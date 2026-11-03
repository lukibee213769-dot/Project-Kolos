use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const QEMU_TIMEOUT: Duration = Duration::from_secs(120);
const POLL_INTERVAL: Duration = Duration::from_millis(200);

/// Locate an OVMF firmware file. Checks env var first, then common paths, then scans directories.
fn find_ovmf(env_var: &str, system_paths: &[&str]) -> PathBuf {
    if let Ok(p) = std::env::var(env_var) {
        let path = PathBuf::from(&p);
        if path.exists() {
            return path;
        }
        eprintln!("WARNING: {env_var}={p} does not exist, falling back to system paths");
    }
    for &p in system_paths {
        let path = PathBuf::from(p);
        if path.exists() {
            return path;
        }
    }
    // Dynamic scan fallback: check directories for any matching .fd file
    let pattern = if env_var.contains("CODE") { "CODE" } else { "VARS" };
    for dir in &["/usr/share/OVMF", "/usr/share/ovmf", "/usr/share/edk2/x64"] {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.ends_with(".fd") && name.contains(pattern) {
                    let path = entry.path();
                    eprintln!("Found OVMF file via directory scan: {}", path.display());
                    return path;
                }
            }
        }
    }
    panic!(
        "Could not find OVMF firmware. Set {env_var} or install the ovmf package.\nChecked: {system_paths:?}"
    );
}

fn spawn_qemu(image: &str, code: &std::path::Path, vars: &std::path::Path, interactive: bool) -> Child {
    let mut command = Command::new("qemu-system-x86_64");
    command
        .args(["-machine", "q35", "-m", "256M", "-serial", "stdio"])
        .args(["-no-reboot", "-no-shutdown"])
        .args(["-drive", &format!("format=raw,file={image}")])
        .args([
            "-drive",
            &format!(
                "if=pflash,format=raw,unit=0,file={},readonly=on",
                code.display()
            ),
        ])
        .args([
            "-drive",
            &format!(
                "if=pflash,format=raw,unit=1,file={},snapshot=on",
                vars.display()
            ),
        ]);

    if interactive {
        // Interactive mode: show display window, no debug-exit, inherit stdio
        command
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
    } else {
        // Automated CI / test mode: headless with isa-debug-exit and piped stdio
        command
            .args(["-display", "none"])
            .args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
    }

    command
        .spawn()
        .expect("start QEMU; install qemu-system-x86_64 first")
}

fn main() {
    let image = env!("KOLOS_UEFI_IMAGE");

    // OVMF Code (read-only firmware ROM) — support both standard 2MB and Ubuntu 24.04+ 4MB filenames
    let code = find_ovmf(
        "KOLOS_OVMF_CODE",
        &[
            "/usr/share/OVMF/OVMF_CODE_4M.fd",
            "/usr/share/OVMF/OVMF_CODE.fd",
            "/usr/share/OVMF/OVMF_CODE.secboot.fd",
            "/usr/share/ovmf/OVMF.fd",
            "/usr/share/edk2/x64/OVMF_CODE.fd",
        ],
    );
    // OVMF Vars (writable NVRAM — snapshot=on so QEMU never modifies the original)
    let vars = find_ovmf(
        "KOLOS_OVMF_VARS",
        &[
            "/usr/share/OVMF/OVMF_VARS_4M.fd",
            "/usr/share/OVMF/OVMF_VARS.fd",
            "/usr/share/OVMF/OVMF_VARS.secboot.fd",
            "/usr/share/ovmf/OVMF_VARS.fd",
            "/usr/share/edk2/x64/OVMF_VARS.fd",
        ],
    );

    let args: Vec<String> = std::env::args().collect();
    let interactive = args.iter().any(|a| a == "--interactive" || a == "-i" || a == "--gui");

    if interactive {
        println!("Starting Kolos OS in interactive GUI mode...");
        let mut child = spawn_qemu(image, &code, &vars, true);
        let status = child.wait().expect("wait on QEMU process");
        println!("QEMU exited with {status}");
        return;
    }

    // Automated CI test mode
    let mut child = spawn_qemu(image, &code, &vars, false);
    let mut serial = child.stdout.take().expect("QEMU stdout is piped");
    let mut child_stdin = child.stdin.take().expect("QEMU stdin is piped");

    let (sender, receiver) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = serial.read_to_end(&mut buffer);
        let _ = sender.send(buffer);
    });

    // Feeder thread: periodically send shell commands and exit until QEMU responds or exits
    thread::spawn(move || {
        for _ in 0..20 {
            thread::sleep(Duration::from_millis(1500));
            let payload = b"\nhelp\nmem\nkolos demo\neval (10 + 20) * 3\necho Kolos_CI_OK\nexit\n";
            if child_stdin.write_all(payload).is_err() {
                break; // Process exited and closed stdin
            }
            let _ = child_stdin.flush();
        }
    });

    let deadline = Instant::now() + QEMU_TIMEOUT;
    let mut timed_out = false;
    let status = loop {
        match child.try_wait().expect("poll QEMU process") {
            Some(status) => break status,
            None if Instant::now() >= deadline => {
                timed_out = true;
                let _ = child.kill();
                break child.wait().expect("reap killed QEMU process");
            }
            None => thread::sleep(POLL_INTERVAL),
        }
    };

    let stdout = receiver.recv().unwrap_or_default();
    let stderr = drain_stderr(&mut child);

    if timed_out {
        eprintln!("Kolos boot timed out after {QEMU_TIMEOUT:?}; QEMU never exited");
    }

    print!("{}", String::from_utf8_lossy(&stdout));
    eprint!("{}", String::from_utf8_lossy(&stderr));

    let stdout_str = String::from_utf8_lossy(&stdout);
    let booted = stdout_str.contains("KOLOS_BOOT_OK");
    // isa-debug-exit maps 0x10 -> (0x10 << 1) | 1 = 33 on Linux
    let expected_exit = matches!(status.code(), Some(33));
    if !booted || !expected_exit {
        eprintln!(
            "Kolos failed to boot: exit_code={:?}, KOLOS_BOOT_OK_marker={}, timed_out={timed_out}",
            status.code(),
            booted,
        );
        std::process::exit(1);
    }
}

fn drain_stderr(child: &mut Child) -> Vec<u8> {
    let Some(mut stderr) = child.stderr.take() else {
        return Vec::new();
    };
    let mut buffer = Vec::new();
    let _ = stderr.read_to_end(&mut buffer);
    buffer
}
