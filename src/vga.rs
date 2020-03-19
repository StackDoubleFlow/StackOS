use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use vga::{
    writers::{Text80x25, TextWriter, ScreenCharacter},
    colors::{Color16Bit, TextModeColor},
};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: TextModeColor::new(Color16Bit::Yellow, Color16Bit::Black),
        text_mode: Text80x25::new(),
    });
}

pub struct Writer {
    column_position: usize,
    color_code: TextModeColor,
    text_mode: Text80x25,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= self.text_mode.get_width() {
                    self.new_line();
                }

                let row = self.text_mode.get_height() - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.text_mode.write_character(col, row, ScreenCharacter::new(byte, color_code));
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        let width = self.text_mode.get_width();
        let height = self.text_mode.get_height();
        for row in 1..height {
            for col in 0..width {
                let character = self.text_mode.read_character(col, row);
                self.text_mode.write_character(col, row - 1, character);
            }
        }
        self.clear_row(height - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenCharacter::new(b' ', self.color_code);
        for col in 0..self.text_mode.get_width() {
            self.text_mode.write_character(col, row, blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
