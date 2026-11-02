/// Raw COM1 serial port driver (no external crates).
/// Works in QEMU via `-serial stdio`.
use core::fmt;
use spin::Mutex;
use x86_64::instructions::port::Port;

const COM1: u16 = 0x3F8;

pub fn init() {
    unsafe {
        // Disable all interrupts
        Port::<u8>::new(COM1 + 1).write(0x00);
        // Enable DLAB to set baud divisor
        Port::<u8>::new(COM1 + 3).write(0x80);
        // Divisor = 1 → 115200 baud
        Port::<u8>::new(COM1).write(0x01_u8);
        Port::<u8>::new(COM1 + 1).write(0x00_u8);
        // 8-N-1, DLAB off
        Port::<u8>::new(COM1 + 3).write(0x03);
        // Enable FIFO, 14-byte threshold
        Port::<u8>::new(COM1 + 2).write(0xC7);
        // RTS + DTR
        Port::<u8>::new(COM1 + 4).write(0x03);
    }
}

/// Write one byte, spin-waiting for TX buffer empty.
pub fn write_byte(byte: u8) {
    unsafe {
        while Port::<u8>::new(COM1 + 5).read() & 0x20 == 0 {
            core::hint::spin_loop();
        }
        Port::<u8>::new(COM1).write(byte);
    }
}

/// Non-blocking read — returns `None` when no byte is available.
pub fn read_byte() -> Option<u8> {
    unsafe {
        if Port::<u8>::new(COM1 + 5).read() & 0x01 != 0 {
            Some(Port::<u8>::new(COM1).read())
        } else {
            None
        }
    }
}

// ── fmt::Write wrapper ────────────────────────────────────────────────────────

struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if b == b'\n' {
                write_byte(b'\r');
            }
            write_byte(b);
        }
        Ok(())
    }
}

static WRITER: Mutex<Writer> = Mutex::new(Writer);

pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).ok();
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => { $crate::serial::_print(format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! serial_println {
    ()             => { $crate::serial_print!("\n") };
    ($($arg:tt)*) => { $crate::serial_print!("{}\n", format_args!($($arg)*)) };
}
