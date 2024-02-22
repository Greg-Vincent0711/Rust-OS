//separate compilation unit - specify not to use std
#![no_std]
//add custom testing framework
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
//needed so that testing frame work uses test_runner as main
#![reexport_test_harness_main = "test_main"]
//allows unstable abi to be used
#![feature(abi_x86_interrupt)]
use core::panic::PanicInfo;
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
// global descriptor table
pub mod gdt;

pub fn init(){
    gdt::init();
    interrupts::init_idt();
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

pub fn test_runner(tests: &[&dyn Testable]){
    serial_println!("Running {} tests", tests.len());
    for test in tests{
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> !{
    serial_println!("[failed]\n");
    serial_println!("[Error info: {}]\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

//lib is it's own separately compiled attribute
// as such it needs it's own entry point
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> !{
    init(); 
    test_main();
    loop{} 
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
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