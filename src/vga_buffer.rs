// Gregory Vincent

#![allow(unused_doc_comments)]
// keep writing to the vga buffer from being optimized
use volatile::Volatile;
use core::fmt;

#[allow(dead_code)]
/**
  * derive - generates implementations for common struct traits
  * debug - type can be printed with println!("{:?}", value)
  * partialeq - can check for equality using ==
  * Eq - support for structural equality
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// repr - controls memory layout of a struct
#[repr(u8)]
pub enum Color{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

//redeclare derive since different structs need different traits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/**
 * ColorCode struct adds no memory overhead
 * always within the bounds of an 8 bit unsigned int
 * this is because it returns a color code equal in size to u8
 */
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode{
    fn new(foreground: Color, background: Color) -> ColorCode {
        /**
         * background left shifted by 4 | foreground == u8
         * ex:
         * fground = 10 = 00001010 = LightGreen
         * bground = 11 = 00001011 = LightCyan
         * bground << 4 == 10110000
         * bground | fground = 10111010
         */
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
/**
 * use C representation of a struct/enum
 * useful when interacting with C code/cross compatibility
 */
#[repr(C)]
struct ScreenCharacter{
    character: u8,
    color_code: ColorCode,
}
//buffer height and width are typically 25 and 80
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

//refers to how memory is laid out - same as it's single field chars
#[repr(transparent)]
struct VgaBuffer{
    //characters is a 2d array 25*80 array of possible characters
    chars: [[Volatile<ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

//used to write to the screen
pub struct Writer{
    column_position: usize,
    color_code: ColorCode,
    /**
     * reference to the Vgabuffer
     * `static lifetime - valid for life of the program
     */
    buffer: &'static mut VgaBuffer,
}

impl Writer{
    //single characters
    pub fn write_byte(&mut self, data_to_write: u8){
        match data_to_write{
            b'\n' => self.new_line(),
            data_to_write => {
                if self.column_position >= BUFFER_WIDTH{
                    self.new_line();
                }
                // if not a newline or at the end of a row, write to buffer
                let col = self.column_position;
                let row = BUFFER_HEIGHT - 1;
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenCharacter{
                    character: data_to_write,
                    color_code
                });
                self.column_position += 1;
            }
        }
    }

    //whole strings
    pub fn write_string(&mut self, s: &str){
        for byte in s.bytes(){
            match byte{
                /**
                 * checks if a printable character or a newline
                 * 0x20..=0x7e checks the byte value within a range
                 * that range being what's readable as a character
                 */
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //otherwise, print a ■ character
                _ => self.write_byte(0xfe)
            }
        }
    }


    fn new_line(&mut self){
        //omit the 0th row since it's off the screen
        for row in 1..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH{
                //shift everything down one
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        // overwrite the original row's memory
        self.clear_row(BUFFER_HEIGHT - 1);
        // reset the column position
        self.column_position = 0;
     }

     fn clear_row(&mut self, row: usize){
        //create a blank character, write it into the vga buffer
        let blank = ScreenCharacter{
            color_code: self.color_code,
            character: b' ',
        };
        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank);
        }
     }
}

//add support for built-in formatting macros for the Writer struct
impl fmt::Write for Writer{
    fn write_str(&mut self, s:&str) -> fmt::Result{
        self.write_string(s);
        Ok(())
    }
}

/**
 * Static WRITER object refers to nonstatic references 
 * This is why the the lazy_static macro is used
 * lazy static is initialized on first use - run time
 * spin is used to ensure memory safety - more in cargo.toml
 */
use spin::Mutex;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VgaBuffer) }
    });
}



// excluded from documentation - not explicitly called
#[doc(hidden)]
pub fn _print(args: fmt::Arguments){
    //use Writ trait without relying on stdlib
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    // closure - keeps deadlock from happening
    // no interrupts can happen while the Writer is locked
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

//builds off print fn
#[macro_export]
macro_rules! print {
    // take an argument, print it using the actual vga buffer print after formatting
    ($($arg:tt)*) => {$crate::vga_buffer::_print(format_args!($($arg)*))};
}

//builds off print macro
#[macro_export]
macro_rules! println {
    //print a newline if called empty
    () => ($crate::print!("\n"));
    //otherwise print it with a newline
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

//vga buffer test
#[test_case]
fn test_println(){
    println!("Testing print to vga buffer");
}

//tests for many lines printing and shifting off page
#[test_case]
fn test_println_many(){
    for _ in 0..150{
        println!("Testing println with a long output");
    }
}

#[test_case]
fn verify_output(){
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    //test by writing something to the screen
    let s = "any random string that fits on a single line.";
    println!("{}", s);
    // keeps possible deadlock from happening
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        // fixes possible race condition between Writer and reading characters
        // allows for writing to a locked writer
        writeln!(writer, "\n{}", s).expect("Writeln failed");
        for(i, c) in s.chars().enumerate(){
            //read back that same screen and compare them
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.character), c);
        }
    })
}
