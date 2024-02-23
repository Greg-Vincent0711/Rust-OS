
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;
use crate::gdt;
// replicates secondary pic slaved to pin 2 on primary pic
use pic8259::ChainedPics;
use spin;
use crate::print;

// IDT must live for program runtime - cpu will reference it a lot
// has to be static but also mutable so that we can set the 
// breakpoint handler fn.
// lazy static allows for the static ref to be evaluated at runtime
// after the breakpoint_handler is set, but still allowing it to be static
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler)
            // point to the gdt index
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        // indexing starts since we're dealing with non-cpu interrupts > 31
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);

        idt
    };
}
 
pub fn init_idt(){
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("Caught a breakpoint exception\n{:#?}", stack_frame);
}

//x86 architecture doesn't allow returning from a double_fault exception
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> !{
    panic!("Caught a double fault exception \n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame){
    print!("Timer interrupt caught");
    // send the EOI signal so we can continue to process timer signals
    unsafe{
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// extern "x86-interrupt" fn keyboard_interrupt(_stack_frame: InterruptStackFrame){
    // do something on keyboard interrupt
    
// }

#[test_case]
fn test_breakpoint_exception(){
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    init_idt();
}

// timer uses first index of pic
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = 
    spin::Mutex::new(unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});


    #[derive(Debug,Clone,Copy)]
    #[repr(u8)]
    pub enum InterruptIndex{
        Timer = PIC_1_OFFSET,
        // Keyboard = PIC_1_OFFSET + 1
    }

    impl InterruptIndex{
        fn as_u8(self) -> u8 {
            self as u8
        }

        fn as_usize(self) -> usize {
            usize::from(self.as_u8())
        }
    }

