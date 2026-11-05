#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod framebuffer;
mod gdt;
mod heap;
mod interrupts;
mod keyboard;
mod serial;
mod shell;
pub mod vm;

use bootloader_api::config::Mapping;
use bootloader_api::{entry_point, BootInfo, BootloaderConfig};

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

const QEMU_EXIT_PORT: u16 = 0xf4;
pub const QEMU_EXIT_SUCCESS: u32 = 0x10;
pub const QEMU_EXIT_FAILURE: u32 = 0x11;

pub fn exit_qemu(code: u32) -> ! {
    unsafe {
        let mut port = x86_64::instructions::port::Port::new(QEMU_EXIT_PORT);
        port.write(code);
    }
    loop {
        x86_64::instructions::hlt();
    }
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // ── 1. Serial (first — needed for debug output from now on) ──────────────
    serial::init();
    serial_println!("[boot] Serial OK");

    // ── 2. GDT + TSS ─────────────────────────────────────────────────────────
    gdt::init();
    serial_println!("[boot] GDT OK");

    // ── 3. IDT + PIC ─────────────────────────────────────────────────────────
    interrupts::init();
    serial_println!("[boot] IDT + PIC OK (interrupts enabled)");

    // ── 4. Heap ───────────────────────────────────────────────────────────────
    let phys_offset = boot_info
        .physical_memory_offset
        .into_option()
        .expect("no physical memory offset in BootInfo");
    heap::init(phys_offset, &boot_info.memory_regions);
    serial_println!("[boot] Heap OK");

    // ── 5. Framebuffer (optional — may be absent in some QEMU configs) ───────
    if let Some(fb) = boot_info.framebuffer.as_mut() {
        framebuffer::init(fb);
        let info = fb.info();
        serial_println!(
            "[boot] Framebuffer OK  {}x{} {:?}  bpp={}",
            info.width,
            info.height,
            info.pixel_format,
            info.bytes_per_pixel,
        );
    } else {
        serial_println!("[boot] No framebuffer — serial-only mode");
    }

    // ── Boot complete marker (checked by CI test runner) ──────────────────────
    serial_println!("AXIOM_BOOT_OK");

    // ── Interactive Shell ─────────────────────────────────────────────────────
    shell::run()
}

// ── Panic handler ─────────────────────────────────────────────────────────────

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Re-init serial in case it was never set up (e.g. very early panic).
    serial::init();
    serial_println!("\n!!! KERNEL PANIC !!!");
    serial_println!("{}", info);
    exit_qemu(QEMU_EXIT_FAILURE)
}
