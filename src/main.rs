// Gregory Vincent Jr
// disables the standard library
#![no_std]
//disable std entry point - libc
#![no_main]
//add custom testing framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
//needed so that testing frame work uses test_runner as main
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
mod vga_buffer;
mod serial;


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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", _info);
    exit_qemu(QemuExitCode::Failed);
    //loop since compiler doesn't know exit_qemu is a program exit
    loop {}
}


//testing with custom-tests framework
#[cfg(test)]
//&[&dyn Fn()] - list to fn trait objects marked with #[test_case]
pub fn test_runner(tests: &[&dyn Testable]){
    serial_println!("Running {} tests", tests.len());
    for test in tests{
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_test(){
    assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    //exit codes don't overlap with QEMU exit codes
    Success = 0x10,
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;
    unsafe{
        let isa_debug_exit_location = 0xf4;
        let mut port = Port::new(isa_debug_exit_location);
        //u32 since iosize is 4 bytes
        port.write(exit_code as u32);
    }
}

// defining a testable trait 

pub trait Testable{
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self){
        serial_print!("{}...\t", core::any::type_name::<T>());
        // invoke the test fn through self since it implements the Fn trait
        self();
        serial_println!("[ok]"); 
    }
}
