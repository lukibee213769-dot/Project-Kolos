use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const QEMU_TIMEOUT: Duration = Duration::from_secs(120);
const POLL_INTERVAL: Duration = Duration::from_millis(200);

/// Locate an OVMF firmware file with Secure Boot disabled.
fn find_ovmf(env_var: &str, system_paths: &[&str]) -> PathBuf {
    if let Ok(p) = std::env::var(env_var) {
        let path = PathBuf::from(&p);
        if path.exists() && !is_secure_boot_firmware(&path) {
            return path;
        }
        eprintln!(
            "WARNING: {env_var}={p} is missing or enables Secure Boot, falling back to system paths"
        );
    }
    for &p in system_paths {
        let path = PathBuf::from(p);
        if path.exists() && !is_secure_boot_firmware(&path) {
            return path;
        }
    }
    // Secure Boot rejects the unsigned UEFI image built by this project.
    let pattern = if env_var.contains("CODE") {
        "CODE"
    } else {
        "VARS"
    };
    for dir in &["/usr/share/OVMF", "/usr/share/ovmf", "/usr/share/edk2/x64"] {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                let path = entry.path();
                if name.ends_with(".fd")
                    && name.contains(pattern)
                    && !is_secure_boot_firmware(&path)
                {
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

fn is_secure_boot_firmware(path: &std::path::Path) -> bool {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    name.contains("secboot")
        || name.contains("snakeoil")
        || name.contains(".ms.")
        || name.ends_with(".ms.fd")
}

fn spawn_qemu(
    image: &str,
    code: &std::path::Path,
    vars: &std::path::Path,
    interactive: bool,
    serial_port: Option<u16>,
) -> Child {
    let mut command = Command::new("qemu-system-x86_64");
    command
        .args(["-machine", "q35", "-m", "256M"])
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
            .args(["-serial", "stdio"])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
    } else {
        let port = serial_port.expect("automated QEMU mode needs a serial port");
        let serial_device =
            format!("socket,id=serial0,host=127.0.0.1,port={port},server=on,wait=off");
        command
            .args(["-chardev", &serial_device, "-serial", "chardev:serial0"])
            .args(["-display", "none"])
            .args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit());
    }

    command
        .spawn()
        .expect("start QEMU; install qemu-system-x86_64 first")
}

fn reserve_serial_port() -> u16 {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .expect("reserve a local TCP port for the QEMU serial console");
    let port = listener
        .local_addr()
        .expect("read the QEMU serial listener address")
        .port();
    drop(listener);
    port
}

fn connect_qemu_serial(port: u16, child: &mut Child) -> TcpStream {
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));
    let deadline = Instant::now() + Duration::from_secs(10);

    loop {
        match TcpStream::connect_timeout(&address, Duration::from_millis(100)) {
            Ok(stream) => return stream,
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::ConnectionRefused
                        | std::io::ErrorKind::TimedOut
                        | std::io::ErrorKind::WouldBlock
                ) => {}
            Err(error) => panic!("connect to QEMU serial console at {address}: {error}"),
        }

        if let Some(status) = child
            .try_wait()
            .expect("poll QEMU while waiting for its serial connection")
        {
            panic!("QEMU exited before opening its serial console at {address}: {status}");
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let status = child
                .wait()
                .expect("reap QEMU after serial connection timeout");
            panic!("QEMU did not open its serial console at {address}: {status}");
        }
    }
}

fn main() {
    let image = env!("AXIOM_UEFI_IMAGE");

    // OVMF Code (read-only firmware ROM) — support both standard 2MB and Ubuntu 24.04+ 4MB filenames
    let code = find_ovmf(
        "AXIOM_OVMF_CODE",
        &[
            "/usr/share/OVMF/OVMF_CODE_4M.fd",
            "/usr/share/OVMF/OVMF_CODE.fd",
            "/usr/share/ovmf/OVMF.fd",
            "/usr/share/edk2/x64/OVMF_CODE.fd",
        ],
    );
    // OVMF Vars (writable NVRAM — snapshot=on so QEMU never modifies the original)
    let vars = find_ovmf(
        "AXIOM_OVMF_VARS",
        &[
            "/usr/share/OVMF/OVMF_VARS_4M.fd",
            "/usr/share/OVMF/OVMF_VARS.fd",
            "/usr/share/ovmf/OVMF_VARS.fd",
            "/usr/share/edk2/x64/OVMF_VARS.fd",
        ],
    );

    let args: Vec<String> = std::env::args().collect();
    let interactive = args
        .iter()
        .any(|a| a == "--interactive" || a == "-i" || a == "--gui");

    if interactive {
        println!("Starting AXIOM OS in interactive GUI mode...");
        let mut child = spawn_qemu(image, &code, &vars, true, None);
        let status = child.wait().expect("wait on QEMU process");
        println!("QEMU exited with {status}");
        return;
    }

    // Automated CI test mode
    let serial_port = reserve_serial_port();
    let mut child = spawn_qemu(image, &code, &vars, false, Some(serial_port));
    let serial = connect_qemu_serial(serial_port, &mut child);
    let mut serial_reader = serial
        .try_clone()
        .expect("clone the QEMU serial stream for output");

    let (sender, receiver) = std::sync::mpsc::channel();
    let (boot_ready_sender, boot_ready_receiver) = std::sync::mpsc::sync_channel(1);
    thread::spawn(move || {
        let mut buffer = Vec::new();
        let mut chunk = [0; 512];
        let marker = b"AXIOM_BOOT_OK";
        let mut marker_sent = false;

        loop {
            match serial_reader.read(&mut chunk) {
                Ok(0) => break,
                Ok(read) => {
                    buffer.extend_from_slice(&chunk[..read]);
                    if !marker_sent && buffer.windows(marker.len()).any(|window| window == marker) {
                        let _ = boot_ready_sender.send(());
                        marker_sent = true;
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(error) => {
                    eprintln!("Failed to read the QEMU serial console: {error}");
                    break;
                }
            }
        }

        if let Err(error) = sender.send(buffer) {
            eprintln!("Failed to deliver QEMU serial output to the smoke test: {error}");
        }
    });

    let mut serial_writer = serial;
    thread::spawn(move || {
        if let Err(error) = boot_ready_receiver.recv_timeout(Duration::from_secs(30)) {
            eprintln!("AXIOM did not reach AXIOM_BOOT_OK before shell input: {error}");
            return;
        }

        thread::sleep(Duration::from_millis(250));
        for command in [
            "help",
            "mem",
            "axiom demo",
            "eval (10+20)*3",
            "echo CI_OK",
            "exit",
        ] {
            let mut line = command.as_bytes().to_vec();
            line.push(b'\n');
            if let Err(error) = serial_writer
                .write_all(&line)
                .and_then(|()| serial_writer.flush())
            {
                eprintln!("Failed to send QEMU shell command {command:?}: {error}");
                return;
            }
            thread::sleep(Duration::from_millis(250));
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

    let stdout = receiver.recv().expect("receive QEMU serial output");
    let stderr = drain_stderr(&mut child);

    if timed_out {
        eprintln!("AXIOM boot timed out after {QEMU_TIMEOUT:?}; QEMU never exited");
    }

    print!("{}", String::from_utf8_lossy(&stdout));
    eprint!("{}", String::from_utf8_lossy(&stderr));

    let stdout_str = String::from_utf8_lossy(&stdout);
    let booted = stdout_str.contains("AXIOM_BOOT_OK");
    let echo_ok = stdout_str.contains("CI_OK");
    let eval_ok = stdout_str.contains("=> 90");
    let demo_ok = stdout_str.contains("Final sum = 55");
    let shell_commands_ok = echo_ok && eval_ok && demo_ok;
    // isa-debug-exit maps 0x10 -> (0x10 << 1) | 1 = 33 on Linux
    let expected_exit = matches!(status.code(), Some(33));
    if !booted || !shell_commands_ok || !expected_exit {
        eprintln!(
            "AXIOM QEMU smoke test failed: exit_code={:?}, AXIOM_BOOT_OK_marker={}, echo_ok={echo_ok}, eval_ok={eval_ok}, demo_ok={demo_ok}, timed_out={timed_out}",
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

#[cfg(test)]
mod tests {
    use super::is_secure_boot_firmware;
    use std::path::Path;

    #[test]
    fn secure_boot_firmware_variants_are_rejected() {
        for path in [
            "/usr/share/OVMF/OVMF_CODE_4M.snakeoil.fd",
            "/usr/share/OVMF/OVMF_VARS_4M.ms.fd",
            "/usr/share/OVMF/OVMF_CODE.secboot.fd",
        ] {
            assert!(is_secure_boot_firmware(Path::new(path)), "{path}");
        }
    }

    #[test]
    fn standard_firmware_variants_are_allowed() {
        for path in [
            "/usr/share/OVMF/OVMF_CODE_4M.fd",
            "/usr/share/OVMF/OVMF_VARS.fd",
        ] {
            assert!(!is_secure_boot_firmware(Path::new(path)), "{path}");
        }
    }
}
