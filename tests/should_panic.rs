#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use learning_os::{QemuExitCode, exit_qemu, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop{}
}

pub fn test_runner(tests: &[&dyn Fn()]){
    serial_println!("Running {} tests", tests.len());
    for test in tests{
        test();
        serial_println!("Didn't panic");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}