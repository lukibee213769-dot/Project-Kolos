/// Kolos OS interactive shell.
///
/// Input: PS/2 keyboard (IRQ1 → `keyboard::next_char()`)
///    or: serial COM1 (non-blocking poll via `serial::read_byte()`)
/// Output: framebuffer (if available) + serial (always)
use alloc::string::String;

// ── Output helpers ────────────────────────────────────────────────────────────

macro_rules! kprint {
    ($($arg:tt)*) => {{
        crate::serial_print!($($arg)*);
        crate::fb_print!($($arg)*);
    }};
}

macro_rules! kprintln {
    ()             => { kprint!("\n") };
    ($($arg:tt)*) => { kprint!("{}\n", format_args!($($arg)*)) };
}

// ── Input: merge keyboard + serial ───────────────────────────────────────────

fn read_char() -> Option<char> {
    // PS/2 keyboard first
    if let Some(c) = crate::keyboard::next_char() {
        return Some(c);
    }
    // Serial (useful when interacting with QEMU via stdio)
    if let Some(b) = crate::serial::read_byte() {
        let c = b as char;
        return Some(c);
    }
    None
}

// ── Command dispatch ──────────────────────────────────────────────────────────

fn run_command(line: &str) {
    let mut parts = line.trim().splitn(2, ' ');
    let cmd = parts.next().unwrap_or("");
    let arg = parts.next().unwrap_or("").trim();

    match cmd {
        "" => {}

        "help" => {
            kprintln!("Kolos OS shell commands:");
            kprintln!("  help            — show this list");
            kprintln!("  echo <text>     — print text");
            kprintln!("  clear           — clear screen");
            kprintln!("  version         — OS + language version");
            kprintln!("  mem             — memory info");
            kprintln!("  reboot          — reboot the machine");
            kprintln!("  exit            — exit QEMU emulator");
        }

        "echo" => {
            kprintln!("{}", arg);
        }

        "clear" => {
            crate::framebuffer::clear();
            // ANSI clear for serial terminal
            crate::serial_print!("\x1B[2J\x1B[H");
        }

        "version" => {
            kprintln!("Kolos OS  v0.2.0-dev  (Rust kernel)");
            kprintln!("Kolos Lang v1.0.1     (MIT)");
            kprintln!("Built with: bootloader 0.11 + x86_64 + pic8259");
        }

        "mem" => {
            let total = crate::heap::total_bytes();
            kprintln!("Heap: {} KiB configured", total / 1024);
        }

        "reboot" => {
            kprintln!("Rebooting...");
            unsafe {
                use x86_64::instructions::port::Port;
                let mut port: Port<u8> = Port::new(0x64);
                port.write(0xFE_u8);
            }
            loop {
                x86_64::instructions::hlt();
            }
        }

        "exit" | "quit" | "qemu-exit" => {
            kprintln!("Shutting down QEMU...");
            crate::exit_qemu(crate::QEMU_EXIT_SUCCESS);
        }

        other => {
            kprintln!("Unknown command: '{}'. Type 'help' for a list.", other);
        }
    }
}

// ── Shell loop ────────────────────────────────────────────────────────────────

pub fn run() -> ! {
    kprintln!();
    kprintln!("  ██╗  ██╗ ██████╗ ██╗      ██████╗ ███████╗");
    kprintln!("  ██║ ██╔╝██╔═══██╗██║     ██╔═══██╗██╔════╝");
    kprintln!("  █████╔╝ ██║   ██║██║     ██║   ██║███████╗");
    kprintln!("  ██╔═██╗ ██║   ██║██║     ██║   ██║╚════██║");
    kprintln!("  ██║  ██╗╚██████╔╝███████╗╚██████╔╝███████║");
    kprintln!("  ╚═╝  ╚═╝ ╚═════╝ ╚══════╝ ╚═════╝ ╚══════╝");
    kprintln!();
    kprintln!("  Kolos OS v0.2.0-dev — type 'help' for commands");
    kprintln!();

    let mut line: String = String::new();
    let mut last_was_cr = false;

    loop {
        kprint!("kolos> ");

        // Collect one line of input
        loop {
            let mut drained_any = false;
            while let Some(c) = read_char() {
                drained_any = true;
                match c {
                    '\n' if last_was_cr => {
                        last_was_cr = false;
                        continue;
                    }
                    '\r' | '\n' => {
                        last_was_cr = c == '\r';
                        kprintln!();
                        run_command(&line);
                        line.clear();
                        break;
                    }
                    '\x08' | '\x7F' => {
                        last_was_cr = false;
                        if !line.is_empty() {
                            line.pop();
                            kprint!("\x08 \x08");
                        }
                    }
                    c if c.is_ascii() && !c.is_ascii_control() => {
                        last_was_cr = false;
                        line.push(c);
                        kprint!("{}", c);
                    }
                    _ => {
                        last_was_cr = false;
                    }
                }
            }
            if line.is_empty() && drained_any {
                // Command was executed
                break;
            }
            if !drained_any {
                x86_64::instructions::hlt();
            }
        }
    }
}
