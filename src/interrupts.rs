use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

use lazy_static::lazy_static;

// IDT must live for program runtime - cpu will reference it a lot
// has to be static but also mutable so that we can set the 
// breakpoint handler fn.
// lazy static allows for the static ref to be evaluated at runtime
// after the breakpoint_handler is set, but still allowing it to be static
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt(){
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("Caught an exception \n{:#?}", stack_frame);
}



#[test_case]
fn test_breakpoint_exception(){
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    init_idt();
}