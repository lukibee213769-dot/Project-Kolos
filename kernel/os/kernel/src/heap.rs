/// Kernel heap — uses the first large-enough usable physical memory region.
use bootloader_api::info::{MemoryRegionKind, MemoryRegions};
use linked_list_allocator::LockedHeap;

/// 2 MiB heap.
pub const HEAP_SIZE: usize = 2 * 1024 * 1024;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init(phys_mem_offset: u64, memory_regions: &MemoryRegions) {
    let region = memory_regions
        .iter()
        .filter(|r| r.kind == MemoryRegionKind::Usable)
        .filter(|r| (r.end - r.start) as usize >= HEAP_SIZE)
        .next()
        .expect("no usable memory region large enough for heap");

    let heap_start = (phys_mem_offset + region.start) as *mut u8;

    // SAFETY: `heap_start` points to a valid, unused physical memory region
    // mapped by the bootloader at `phys_mem_offset`.
    unsafe {
        ALLOCATOR.lock().init(heap_start, HEAP_SIZE);
    }

    crate::serial_println!(
        "[heap] {:#x}..{:#x} ({} KiB)",
        region.start,
        region.start + HEAP_SIZE as u64,
        HEAP_SIZE / 1024
    );
}

pub fn total_bytes() -> usize {
    HEAP_SIZE
}
