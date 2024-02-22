/**
 * gdt == Global Descriptor Table
 * Creating a Task State Segment to use an IST
 * The IST has a list of known good stack pointers
 * We need one to switch stacks to stop a fatal
 * Triple fault from happening when the stack ptr
 * is stuck on the guard page after stack overflow
 */

 //creating the tss
 use x86_64::VirtAddr;
 use x86_64::structures::tss::TaskStateSegment;
 use lazy_static::lazy_static;
 //creating the gdt
 use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
 //reloading the code segment register and making the TSS used by the cpu
 use x86_64::structures::gdt::SegmentSelector;

 pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
 lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // defining the IST entry for double fault
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            // static muts are susceptible to race conditions hence the unsafe
            let stack_start = VirtAddr::from_ptr(unsafe{&STACK});
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static!{
    /**
     * needed to make the cpu use the TSS on stack overflow
     * so that it can switch to a different stack in the IST
     */
    static ref GDT: (GlobalDescriptorTable,Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        //this reloads the code and tss segment registers on the GDT
        let code_segment_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_segment_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors{code_segment_selector, tss_segment_selector})
    };
}

pub fn init(){
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    GDT.0.load();
    unsafe{
        CS::set_reg(GDT.1.code_segment_selector);
        load_tss(GDT.1.tss_segment_selector);
    }
}


struct Selectors{
    code_segment_selector: SegmentSelector,
    tss_segment_selector: SegmentSelector,
}


