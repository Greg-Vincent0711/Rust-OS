// integration tests are separate executables
//so re supply attributes below
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(learning_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use learning_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    test_main();
    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    learning_os::test_panic_handler(info);
}

//example test case
#[test_case]
fn test_println(){
    println!("Println macsro is working")
}