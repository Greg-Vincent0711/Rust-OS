// Gregory Vincent Jr
// disables the standard library
#![no_std]
//disable std entry point - libc
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(learning_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use learning_os::println;


// To build for QEMU -  cargo bootimage; 
// To run with QEMU - qemu-system-x86_64 -drive format=raw,file=target/x86_64-buildData/debug/bootimage-learning_os.bin
// To test - cargo test

/**
 * new entry point - no runtime is calling main anymore
 * 
 * disable name mangling - needed to tell
 * the entry point fn name to the linker
 * extern C - uses C calling convention
 */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Universe{}", "!");
    //initialize the idt, set the breakpoint handler
    learning_os::init();

    // triggering a page fault to understand paging errors
    let ptr = 0x205280 as *mut u8;
    unsafe {let x = *ptr;}
    println!("Successful reading");

    // will throw an error since we can't write to a code page
    // Hence the Protection_Violation Error Code
    unsafe {*ptr = 42;}
    learning_os::hlt_loop();
    //invoke a breakpoint exception to test the handler
    // x86_64::instructions::interrupts::int3();

    //triggering a page fault to catch a double fault
    // unsafe{
    //     //virtual addr isn't mapped to a physical, so a fault occurs
    //     *(0xdeadbeef as *mut u8) = 42;
    // }    

    /*
     * test_main is conditionally compiled - hence the cfg flag
     * Only when the test_runner fn is in effect
     */
    // #[cfg(test)]
    // test_main();
    // println!("Successfully caught a breakpoint and didn't crash.");
}


// non-test panic handler
#[cfg(not(test))]
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    // post output in qemu 
    println!("{}", _info);
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    learning_os::hlt_loop();
}

