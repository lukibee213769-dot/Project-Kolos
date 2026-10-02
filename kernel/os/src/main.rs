use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};
use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const QEMU_TIMEOUT: Duration = Duration::from_secs(120);
const POLL_INTERVAL: Duration = Duration::from_millis(200);

fn spawn_qemu(image: &str, code: &std::path::Path, vars: &std::path::Path) -> Child {
    let mut command = Command::new("qemu-system-x86_64");
    command
        .args(["-machine", "q35", "-m", "256M", "-serial", "stdio"])
        .args(["-display", "none", "-no-reboot", "-no-shutdown"])
        .args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"])
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
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
        .spawn()
        .expect("start QEMU; install qemu-system-x86_64 first")
}

fn main() {
    let image = env!("KOLOS_UEFI_IMAGE");
    let firmware =
        Prebuilt::fetch(Source::LATEST, "target/ovmf").expect("download or locate OVMF firmware");
    let code = firmware.get_file(Arch::X64, FileType::Code);
    let vars = firmware.get_file(Arch::X64, FileType::Vars);

    let mut child = spawn_qemu(image, &code, &vars);
    let mut serial = child.stdout.take().expect("QEMU stdout is piped");

    // The kernel exits QEMU through isa-debug-exit. If it never reaches that
    // port (boot failure or firmware hang) we must not block CI forever, so the
    // serial console is drained on a helper thread while this thread polls the
    // process and kills it once the timeout elapses. No shared lock is involved,
    // which is what previously deadlocked the watchdog.
    let (sender, receiver) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = serial.read_to_end(&mut buffer);
        let _ = sender.send(buffer);
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

    let booted = String::from_utf8_lossy(&stdout).contains("KOLOS_BOOT_OK");
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
