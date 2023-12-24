// Gregory Vincent
//disables the standard library
#![no_std]
//disable std entry point - libc
#![no_main]
use core::panic::PanicInfo;


// TO RUN WITH QEMU - qemu-system-x86_64 -drive format=raw,file=target/x86_64-buildData/debug/

/**
 * new entry point - no runtime is calling main anymore
 * 
 * disable name mangling - needed to tell
 * the entry point fn name to the linker
 * 
 * extern C - uses C calling convention
 */
static TEST_MESSAGE: &[u8] = b"Hello, this is a basic message to the screen.!";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //mutable raw pointer(memory/ownership is managed by the user) to the beginning of the VGA buffer
    let vga_buffer = 0xb8000 as *mut u8;
    //for each byte of data in our message
    for (i, &character_bytedata) in TEST_MESSAGE.iter().enumerate() {
        //code in this block can break safety guarantees
        unsafe {
            //write this character into the vga buffer at the position of the buffer
            *vga_buffer.offset(i as isize * 2) = character_bytedata;
            // increment the pointer for the next character we're writing to the buffer
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop{}
}



/// Panic handler for undefined behavior happens
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    //if there's a panic, loop indefinitely
    loop {}
}
