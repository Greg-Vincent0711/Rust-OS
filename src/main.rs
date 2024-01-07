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


// TO RUN WITH QEMU -  cargo bootimage; qemu-system-x86_64 -drive format=raw,file=target/x86_64-buildData/debug/bootimage-learning_os.bin
// TO TEST - cargo test
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
    /*
     * test_main is conditionally compiled
     * Only when the test_runner fn is in effect
     */
    #[cfg(test)]
    test_main();
    loop{}
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
    learning_os::test_panic_handler(_info);
}

