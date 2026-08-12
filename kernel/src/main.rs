mod scheduler;
mod memory;

fn main() {
    println!("Kolos kernel prototype: startup sequence")
;
    memory::init();
    scheduler::run();
}
