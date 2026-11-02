/// Global Descriptor Table + Task State Segment.
/// Required before enabling interrupts (double-fault handler needs IST entry).
use spin::Lazy;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST: u16 = 0;
const STACK_SIZE: usize = 4096 * 5; // 20 KiB double-fault stack

static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST as usize] = {
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        // SAFETY: only ever read as a pointer to set the IST entry.
        let start = VirtAddr::from_ptr(unsafe { core::ptr::addr_of!(STACK) });
        start + STACK_SIZE as u64
    };
    tss
});

struct Selectors {
    code: SegmentSelector,
    tss: SegmentSelector,
}

static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let code = gdt.append(Descriptor::kernel_code_segment());
    let tss = gdt.append(Descriptor::tss_segment(&TSS));
    (gdt, Selectors { code, tss })
});

pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code);
        load_tss(GDT.1.tss);
    }
}

