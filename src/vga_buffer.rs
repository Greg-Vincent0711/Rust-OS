// Gregory Vincent
#![allow(unused_doc_comments)]

/**
  * struct must be represented as an unsigned 8bit int
  * repr - controls memory layout of a type
*/

/**
  * derive - generates implementations for common struct traits
  * debug - type can be printed with println!("{:?}", value)
  * partialeq - can check for equality using ==
  * Eq - support for structural equality
*/
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

//redeclare derive since different structs need different fns
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

#[derive(Debug, Clone, PartialEq, Eq)]
/**
 * use C representation of a struct/enum
 * useful when interacting with C code
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
    chars: [[ScreenCharacter; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
                self.buffer.chars[row][col] = ScreenCharacter{
                    character: data_to_write,
                    color_code
                };
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

    //todo - implement
    fn new_line(&mut self){ }
}

pub fn test_write() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut VgaBuffer) },
    };
    writer.write_byte(b'h');
    writer.write_string("ello there");

}