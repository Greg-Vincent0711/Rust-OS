// Gregory Vincent Jr
// needed to send data to the host system from kernel
// so we can see console output
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let first_serial_interface_port = 0x3F8;
        let mut serial_port = unsafe {SerialPort::new(first_serial_interface_port)};
        serial_port.init();
        Mutex::new(serial_port)
    };
}
// macros to make serial port more usable

//under the hood print fn each macro calls
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments){
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Serial printing failed.");
}


#[macro_export]
macro_rules! serial_print {
    //print the arg that's passed
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}


#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

