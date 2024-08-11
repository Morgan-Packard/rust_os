use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
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

pub struct Screen {
    pub fg: Color,
    pub bg: Color,
    pub row: usize,
    pub col: usize,
}

const ROW_MAX: usize = 25;
const COL_MAX: usize = 80;

impl Screen {

    pub fn print_char(&mut self, c: u8) {
        let buffer = 0xb8000 as *mut u8;
        if c == b'\n' {
            self.row += 1;
            self.col = 0;
        }
        else {
            
            let offset = self.row * COL_MAX + self.col;
            unsafe {
                *buffer.add(offset * 2) = c;
                *buffer.add(offset * 2 + 1) = 
                    ((self.bg as u8) << 4) + self.fg as u8;
            }
            self.col += 1;
            if self.col >= COL_MAX {
                self.col = 0;
                self.row += 1;
            }
        }
        
        if self.row >= ROW_MAX {
            for row in 1..ROW_MAX {
                for col in 0..COL_MAX {
                    let offset = row * COL_MAX + col;
                    let new_offset = offset - COL_MAX;
                    unsafe {
                        *buffer.add(new_offset * 2) = 
                            *buffer.add(offset * 2);

                        *buffer.add(new_offset * 2 + 1) = 
                            *buffer.add(offset * 2 + 1);
                    }
                }
            }
            
            let row = ROW_MAX - 1;
            for col in 0..COL_MAX {
                let offset = row * COL_MAX + col;
                unsafe {
                    *buffer.add(offset * 2) = 0;
                    *buffer.add(offset * 2 + 1) = 0
                }
            }
            self.row -= 1;
        }
    }

    pub fn print_string(&mut self, s: &str) {
        for c in s.bytes() {
            self.print_char(c);
        }
    }
}

pub static SCREEN: Mutex<Screen> = Mutex::new( Screen {
    fg: Color::Red,
    bg: Color::Black,
    row: 0,
    col: 0
});

impl fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s);
        return Ok(());
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts::without_interrupts;

    without_interrupts(|| SCREEN.lock().write_fmt(args).unwrap());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::printing::_print(format_args!($($arg)*)));
}


#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

