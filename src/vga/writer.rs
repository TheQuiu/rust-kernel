use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::vga::color::{Color, ColorCode};
use crate::vga::vga_buffer::{Buffer, ScreenChar};

lazy_static! {
    pub static ref DEFAULT_WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub struct Writer {
    column_position: usize,
    pub(crate) color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(
        column_position: usize,
        color_code: ColorCode,
        buffer: &'static mut Buffer,
    ) -> Self {
        Writer {
            column_position,
            color_code,
            buffer,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.write_with_color(byte, self.color_code);
    }

    pub fn write_with_color(
        &mut self,
        byte: u8,
        color_code: ColorCode,
    ) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.write_byte(row, col, byte, color_code);
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        self.write_string_with_color(s, self.color_code);
    }

    pub fn write_string_with_color(
        &mut self,
        s: &str,
        color_code: ColorCode,
    ) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_with_color(byte, color_code),
                _ => self.write_with_color(0xfe, color_code),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.read_byte(row, col);
                self.buffer.write_byte(row - 1, col, character, self.color_code);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);
        for col in 0..BUFFER_WIDTH {
            self.buffer.write_byte(row, col, blank.ascii_character, blank.color_code);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}