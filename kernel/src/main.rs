mod scheduler;
mod memory;
pub mod vm;

use vm::{Instruction, Value, Vm};

fn main() {
    println!("========================================");
    println!("    Kolos Kernel & Runtime OS Prototype  ");
    println!("========================================");

    // 1. Initialize Memory Manager
    memory::init();

    // 2. Initialize Scheduler
    scheduler::run();

    // 3. Embedded Kolos VM Execution
    println!("\n[Kernel:VM] Initializing embedded Kolos VM...");

    // Bytecode for: let x = 5; let y = 10; x + y * 2;
    let program = vec![
        Instruction::Push(Value::Int(5)),
        Instruction::Store("x".to_string()),
        Instruction::Push(Value::Int(10)),
        Instruction::Store("y".to_string()),
        Instruction::Load("x".to_string()),
        Instruction::Load("y".to_string()),
        Instruction::Push(Value::Int(2)),
        Instruction::Mul,
        Instruction::Add,
        Instruction::Print,
        Instruction::Halt,
    ];

    let mut machine = Vm::new(program);
    match machine.run() {
        Ok(Some(result)) => {
            println!("[Kernel:VM] Program completed successfully with result: {}", result);
        }
        Ok(None) => {
            println!("[Kernel:VM] Program completed with no return value");
        }
        Err(e) => {
            println!("[Kernel:VM] Execution error: {:?}", e);
        }
    }

    println!("========================================");
    println!("    Kolos Kernel shutdown complete       ");
    println!("========================================");
}
