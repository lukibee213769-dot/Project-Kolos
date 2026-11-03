/// Kolos OS interactive shell.
///
/// Input: PS/2 keyboard (IRQ1 → `keyboard::next_char()`)
///    or: serial COM1 (non-blocking poll via `serial::read_byte()`)
/// Output: framebuffer (if available) + serial (always)
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use crate::vm::{Instruction, Value, Vm};

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
    if let Some(c) = crate::keyboard::next_char() {
        return Some(c);
    }
    if let Some(b) = crate::serial::read_byte() {
        return Some(b as char);
    }
    None
}

// ── Expression compiler (Shunting-yard algorithm to Kolos Bytecode) ──────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(i64),
    Op(char),
    LParen,
    RParen,
}

fn tokenize(expr: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expr.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' => i += 1,
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            '+' | '-' | '*' | '/' | '%' | '^' => {
                tokens.push(Token::Op(chars[i]));
                i += 1;
            }
            c if c.is_ascii_digit() => {
                let mut num: i64 = 0;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    num = num * 10 + (chars[i] as i64 - '0' as i64);
                    i += 1;
                }
                tokens.push(Token::Num(num));
            }
            _ => return Err("unexpected character in expression"),
        }
    }
    Ok(tokens)
}

fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn compile_expr(tokens: &[Token]) -> Result<Vec<Instruction>, &'static str> {
    let mut bytecode = Vec::new();
    let mut op_stack = Vec::new();

    for token in tokens {
        match token {
            Token::Num(n) => {
                bytecode.push(Instruction::Push(Value::Int(*n)));
            }
            Token::Op(op) => {
                while let Some(&Token::Op(top_op)) = op_stack.last() {
                    if precedence(top_op) >= precedence(*op) {
                        emit_op(&mut bytecode, top_op);
                        op_stack.pop();
                    } else {
                        break;
                    }
                }
                op_stack.push(Token::Op(*op));
            }
            Token::LParen => op_stack.push(Token::LParen),
            Token::RParen => {
                let mut found_matching = false;
                while let Some(top) = op_stack.pop() {
                    match top {
                        Token::Op(op) => emit_op(&mut bytecode, op),
                        Token::LParen => {
                            found_matching = true;
                            break;
                        }
                        _ => {}
                    }
                }
                if !found_matching {
                    return Err("mismatched parentheses");
                }
            }
        }
    }

    while let Some(top) = op_stack.pop() {
        match top {
            Token::Op(op) => emit_op(&mut bytecode, op),
            Token::LParen | Token::RParen => return Err("mismatched parentheses"),
            _ => {}
        }
    }

    bytecode.push(Instruction::Halt);
    Ok(bytecode)
}

fn emit_op(bytecode: &mut Vec<Instruction>, op: char) {
    match op {
        '+' => bytecode.push(Instruction::Add),
        '-' => bytecode.push(Instruction::Sub),
        '*' => bytecode.push(Instruction::Mul),
        '/' => bytecode.push(Instruction::Div),
        '%' => bytecode.push(Instruction::Mod),
        '^' => bytecode.push(Instruction::Pow),
        _ => {}
    }
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
            kprintln!("  eval <expr>     — evaluate math expr via Kolos VM (e.g. eval 2 + 3 * 4)");
            kprintln!("  kolos demo      — run Kolos VM loop on bare metal (sum 1..10)");
            kprintln!("  kolos info      — show VM instruction set status");
            kprintln!("  echo <text>     — print text");
            kprintln!("  clear           — clear screen");
            kprintln!("  version         — OS + language version");
            kprintln!("  mem             — memory info");
            kprintln!("  reboot          — reboot the machine");
            kprintln!("  exit            — exit QEMU emulator");
        }

        "eval" => {
            if arg.is_empty() {
                kprintln!("Usage: eval <expr>  (example: eval (10 + 5) * 2)");
                return;
            }
            match tokenize(arg).and_then(|tokens| compile_expr(&tokens)) {
                Ok(bytecode) => {
                    let mut vm = Vm::new(bytecode);
                    match vm.run() {
                        Ok(Some(val)) => kprintln!("=> {}", val),
                        Ok(None) => kprintln!("=> nil"),
                        Err(e) => kprintln!("Kolos VM Error: {:?}", e),
                    }
                }
                Err(err) => kprintln!("Syntax error: {}", err),
            }
        }

        "kolos" => {
            match arg {
                "demo" => {
                    kprintln!("Running Kolos VM bare-metal demo: loop sum(1..10)...");
                    // sum = 0; i = 1; while i <= 10 { sum += i; i += 1 }; return sum
                    let program = vec![
                        Instruction::Push(Value::Int(0)),
                        Instruction::Store(String::from("sum")),
                        Instruction::Push(Value::Int(1)),
                        Instruction::Store(String::from("i")),
                        // loop (ip = 4)
                        Instruction::Load(String::from("i")),
                        Instruction::Push(Value::Int(10)),
                        Instruction::Le,
                        Instruction::Jz(17),
                        Instruction::Load(String::from("sum")),
                        Instruction::Load(String::from("i")),
                        Instruction::Add,
                        Instruction::Store(String::from("sum")),
                        Instruction::Load(String::from("i")),
                        Instruction::Push(Value::Int(1)),
                        Instruction::Add,
                        Instruction::Store(String::from("i")),
                        Instruction::Jmp(4),
                        // exit (ip = 17)
                        Instruction::Load(String::from("sum")),
                        Instruction::Print,
                        Instruction::Halt,
                    ];
                    let mut vm = Vm::new(program);
                    match vm.run() {
                        Ok(Some(res)) => kprintln!("=> Demo finished successfully! Final sum = {}", res),
                        Ok(None) => kprintln!("=> Demo finished with nil"),
                        Err(e) => kprintln!("=> VM error: {:?}", e),
                    }
                }
                "info" => {
                    kprintln!("Kolos Bytecode Virtual Machine (Bare-Metal x86-64)");
                    kprintln!("  Engine: Stack-based Bytecode VM");
                    kprintln!("  Instructions supported: 30 ops (Arithmetic, Logic, Stack, Jumps, Calls)");
                    kprintln!("  Memory backend: 2 MiB UEFI identity-mapped heap");
                    kprintln!("  Architecture: no_std, bare metal, zero host dependencies");
                }
                _ => {
                    kprintln!("Usage: kolos demo | kolos info");
                }
            }
        }

        "echo" => {
            kprintln!("{}", arg);
        }

        "clear" => {
            crate::framebuffer::clear();
            crate::serial_print!("\x1B[2J\x1B[H");
        }

        "version" => {
            kprintln!("Kolos OS  v0.2.0-dev  (Rust bare-metal kernel)");
            kprintln!("Kolos VM  v1.0.1     (In-kernel Bytecode Engine)");
            kprintln!("Kolos Lang v1.0.1     (MIT)");
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
                break;
            }
            if !drained_any {
                x86_64::instructions::hlt();
            }
        }
    }
}
