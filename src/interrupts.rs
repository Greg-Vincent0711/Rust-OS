
use core::iter::Scan;

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
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

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
    print!(".");
    unsafe{
        // send the EOI signal so we can continue to process other signals
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame){
    use x86_64::instructions::port::Port;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
   lazy_static!{
    // Us104Key - standard keyboard, scancodes, ignore the ctrl key for now
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
   }
   // create a reference to the locked keyboard object
   let mut keyboard = KEYBOARD.lock();
   let mut port = Port::new(0x60);
   // read the scancode from the hardware port attached to the keyboard
   let scancode: u8 = unsafe{port.read()};
   // bind the scancode to the keyboard if its there
   if let Ok(Some(key_event)) = keyboard.add_byte(scancode){
    // if there's a scancode, process its data...is it a press or release, and the key
    if let Some(key) = keyboard.process_keyevent(key_event){
        match key {
            // if we have a readable character, print it, etc
            DecodedKey::Unicode(readable_character) => print!("{}", readable_character),
            DecodedKey::RawKey(character) => print!("{:?}", character)
        }
    }
   }
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

#[test_case]
fn test_breakpoint_exception(){
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    init_idt();
}

// timer uses first index of pic
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});


#[derive(Debug,Clone,Copy)]
#[repr(u8)]
pub enum InterruptIndex{
    Timer = PIC_1_OFFSET,
    Keyboard
}

impl InterruptIndex{
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

