// Gregory Vincent
//disables the standard library
#![no_std]
//disable std entry point - libc
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;


// TO RUN WITH QEMU -  cargo bootimage; qemu-system-x86_64 -drive format=raw,file=target/x86_64-buildData/debug/bootimage-learning_os.bin

/**
 * new entry point - no runtime is calling main anymore
 * 
 * disable name mangling - needed to tell
 * the entry point fn name to the linker
 * 
 * extern C - uses C calling convention
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Universe{}", "!");
    panic!("Some message");
}


/// Panic handler for undefined behavior happens
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    //we can actually print something now
    println!("{}", _info);
    loop {}
}

