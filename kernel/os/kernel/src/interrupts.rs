/// IDT setup + PIC (8259) remapping + interrupt handlers.
use crate::gdt;
use pic8259::ChainedPics;
use spin::{Lazy, Mutex};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

// ── PIC offsets ───────────────────────────────────────────────────────────────
// Remap PICs away from CPU exceptions (0-31).
pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum IrqVector {
    Timer = PIC1_OFFSET,
    Keyboard = PIC1_OFFSET + 1,
}

pub static PICS: Mutex<ChainedPics> =
    // SAFETY: offsets 32/40 are outside the CPU exception range.
    Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

// ── IDT ───────────────────────────────────────────────────────────────────────

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    // CPU exceptions
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST);
    }

    // Hardware IRQs — IndexMut<u8> is implemented on IDT
    idt[IrqVector::Timer as u8].set_handler_fn(timer_handler);
    idt[IrqVector::Keyboard as u8].set_handler_fn(keyboard_handler);

    idt
});

pub fn init() {
    IDT.load();
    // SAFETY: PIC offsets are valid and don't overlap with exception vectors.
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

// ── Exception handlers ────────────────────────────────────────────────────────

extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    crate::serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", frame);
}

extern "x86-interrupt" fn page_fault_handler(
    frame: InterruptStackFrame,
    code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    crate::serial_println!(
        "EXCEPTION: PAGE FAULT\n  address: {:?}\n  code: {:?}\n{:#?}",
        Cr2::read(),
        code,
        frame
    );
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, _code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", frame);
}

// ── IRQ handlers ──────────────────────────────────────────────────────────────

extern "x86-interrupt" fn timer_handler(_frame: InterruptStackFrame) {
    // Tick: acknowledge PIC interrupt.
    unsafe { PICS.lock().notify_end_of_interrupt(IrqVector::Timer as u8) };
}

extern "x86-interrupt" fn keyboard_handler(_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    let scancode: u8 = unsafe { Port::new(0x60_u16).read() };
    crate::keyboard::push_scancode(scancode);
    unsafe { PICS.lock().notify_end_of_interrupt(IrqVector::Keyboard as u8) };
}
