/// Physical and Virtual Memory Management for Kolos Kernel.

pub const PAGE_SIZE: usize = 4096;
pub const TOTAL_PAGES: usize = 1024; // 4MB prototype memory space

#[allow(dead_code)]
pub struct FrameAllocator {
    used_frames: [bool; TOTAL_PAGES],
    free_count: usize,
}

#[allow(dead_code)]
impl FrameAllocator {
    pub const fn new() -> Self {
        Self {
            used_frames: [false; TOTAL_PAGES],
            free_count: TOTAL_PAGES,
        }
    }

    pub fn alloc_frame(&mut self) -> Option<usize> {
        for (i, used) in self.used_frames.iter_mut().enumerate() {
            if !*used {
                *used = true;
                self.free_count -= 1;
                return Some(i * PAGE_SIZE);
            }
        }
        None
    }

    pub fn free_frame(&mut self, addr: usize) {
        let index = addr / PAGE_SIZE;
        if index < TOTAL_PAGES && self.used_frames[index] {
            self.used_frames[index] = false;
            self.free_count += 1;
        }
    }

    pub fn free_frames_count(&self) -> usize {
        self.free_count
    }
}

pub fn init() {
    println!("[Kernel:Memory] Initializing Frame Allocator (4MB prototype, {} pages)", TOTAL_PAGES);
    let mut allocator = FrameAllocator::new();
    let frame1 = allocator.alloc_frame().expect("Failed to alloc initial page");
    println!("[Kernel:Memory] First page allocated at 0x{:08X}, free pages: {}", frame1, allocator.free_frames_count());
    allocator.free_frame(frame1);
    println!("[Kernel:Memory] Page 0x{:08X} freed, free pages: {}", frame1, allocator.free_frames_count());
}
