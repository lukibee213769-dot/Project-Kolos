use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};
use std::process::Command;

fn main() {
    let image = env!("KOLOS_UEFI_IMAGE");
    let firmware =
        Prebuilt::fetch(Source::LATEST, "target/ovmf").expect("download or locate OVMF firmware");
    let code = firmware.get_file(Arch::X64, FileType::Code);
    let vars = firmware.get_file(Arch::X64, FileType::Vars);

    let output = Command::new("qemu-system-x86_64")
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
        .output()
        .expect("start QEMU; install qemu-system-x86_64 first");

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    let booted = String::from_utf8_lossy(&output.stdout).contains("KOLOS_BOOT_OK");
    let expected_exit = matches!(output.status.code(), Some(33) | Some(16));
    if !booted || !expected_exit {
        eprintln!(
            "Kolos failed to boot: status {:?}, marker={booted}",
            output.status.code()
        );
        std::process::exit(1);
    }
}
