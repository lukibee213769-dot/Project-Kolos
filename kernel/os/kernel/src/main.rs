#![no_std]
#![no_main]

use bootloader_api::{BootInfo, entry_point};
use core::fmt::Write;
use uart_16550::backend::PioBackend;
use uart_16550::{Config, Uart16550Tty};

const QEMU_EXIT_PORT: u16 = 0xf4;
const QEMU_EXIT_SUCCESS: u32 = 0x10;
const QEMU_EXIT_FAILURE: u32 = 0x11;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let mut serial = serial();
    let usable_regions = boot_info
        .memory_regions
        .iter()
        .filter(|region| region.kind == bootloader_api::info::MemoryRegionKind::Usable)
        .count();

    let _ = writeln!(serial, "Kolos OS kernel started");
    let _ = writeln!(
        serial,
        "Boot memory regions: {}",
        boot_info.memory_regions.len()
    );
    let _ = writeln!(serial, "Usable memory regions: {usable_regions}");
    let _ = writeln!(serial, "KOLOS_BOOT_OK");
    exit_qemu(QEMU_EXIT_SUCCESS)
}

fn serial() -> Uart16550Tty<PioBackend> {
    // SAFETY: COM1 is the conventional x86 serial port used by QEMU's serial backend.
    unsafe { Uart16550Tty::new_port(0x3f8, Config::default()) }
        .expect("initialize COM1 serial output")
}

fn exit_qemu(code: u32) -> ! {
    // SAFETY: QEMU's isa-debug-exit device is configured at this I/O port in the runner.
    unsafe {
        let mut port = x86_64::instructions::port::Port::new(QEMU_EXIT_PORT);
        port.write(code);
    }
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = writeln!(serial(), "Kolos kernel panic: {info}");
    exit_qemu(QEMU_EXIT_FAILURE)
}
