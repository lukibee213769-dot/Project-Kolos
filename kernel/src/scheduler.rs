/// Multi-tasking scheduler for Kolos Kernel.

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Terminated,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub state: TaskState,
}

pub struct Scheduler {
    tasks: VecDeque<Task>,
    next_id: u32,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
            next_id: 1,
        }
    }

    pub fn spawn(&mut self, name: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let task = Task {
            id,
            name: name.to_string(),
            state: TaskState::Ready,
        };
        self.tasks.push_back(task);
        println!("[Kernel:Scheduler] Spawned task #{} ({})", id, name);
        id
    }

    pub fn run(&mut self) {
        println!("[Kernel:Scheduler] Starting round-robin task execution");
        while let Some(mut task) = self.tasks.pop_front() {
            task.state = TaskState::Running;
            println!("[Kernel:Scheduler] Executing task #{} ({})", task.id, task.name);
            task.state = TaskState::Terminated;
        }
        println!("[Kernel:Scheduler] All tasks completed");
    }
}

pub fn run() {
    let mut scheduler = Scheduler::new();
    scheduler.spawn("idle");
    scheduler.spawn("kolos_vm_worker");
    scheduler.run();
}
