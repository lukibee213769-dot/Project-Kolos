use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const QEMU_TIMEOUT: Duration = Duration::from_secs(120);

fn main() {
    let image = env!("KOLOS_UEFI_IMAGE");
    let firmware =
        Prebuilt::fetch(Source::LATEST, "target/ovmf").expect("download or locate OVMF firmware");
    let code = firmware.get_file(Arch::X64, FileType::Code);
    let vars = firmware.get_file(Arch::X64, FileType::Vars);

    let child = Command::new("qemu-system-x86_64")
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
        .stderr(Stdio::piped())
        .spawn()
        .expect("start QEMU; install qemu-system-x86_64 first");

    // The kernel exits QEMU through isa-debug-exit. If it never reaches that
    // port (boot failure or firmware hang) we must not block CI forever, so a
    // watchdog kills the VM once the timeout elapses.
    let child = Arc::new(Mutex::new(Some(child)));
    let watchdog_child = Arc::clone(&child);
    let timed_out = Arc::new(Mutex::new(false));
    let watchdog_flag = Arc::clone(&timed_out);
    thread::spawn(move || {
        thread::sleep(QEMU_TIMEOUT);
        let mut guard = watchdog_child.lock().expect("QEMU handle mutex");
        if let Some(mut qemu) = guard.take() {
            *watchdog_flag.lock().expect("timeout flag mutex") = true;
            let _ = qemu.kill();
            let _ = qemu.wait();
        }
    });

    let output = {
        let mut guard = child.lock().expect("QEMU handle mutex");
        let mut qemu = guard.take().expect("QEMU process handle");
        qemu.wait_with_output().expect("wait for QEMU to finish")
    };

    if *timed_out.lock().expect("timeout flag mutex") {
        eprintln!("Kolos boot timed out after {QEMU_TIMEOUT:?}; QEMU never exited");
        std::process::exit(1);
    }

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    let booted = String::from_utf8_lossy(&output.stdout).contains("KOLOS_BOOT_OK");
    // isa-debug-exit maps 0x10 → (0x10 << 1) | 1 = 33 on Linux
    let expected_exit = matches!(output.status.code(), Some(33));
    if !booted || !expected_exit {
        eprintln!(
            "Kolos failed to boot: exit_code={:?}, KOLOS_BOOT_OK_marker={}",
            output.status.code(),
            booted,
        );
        std::process::exit(1);
    }
}
