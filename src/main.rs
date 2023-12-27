// Gregory Vincent Jr
// disables the standard library
#![no_std]
//disable std entry point - libc
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;


// TO RUN WITH QEMU - qemu-system-x86_64 -drive format=raw,file=target/x86_64-buildData/debug/bootimage-learning_os.bin

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
    use core::fmt::Write;
    /*
     * interior mutability is allowed while also being mem safe, since we're locking
     * unwrap is again used since fmtWrite returns a Return type, which we have to use
     */
    vga_buffer::WRITER.lock().write_str("Hello in a new way.").unwrap();
    write!(vga_buffer::WRITER.lock(), "Numbers: {} {}", 50, 29.2020).unwrap();
    loop{}
}


/// Panic handler for undefined behavior happens
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    //if there's a panic, loop indefinitely
    loop {}
}

