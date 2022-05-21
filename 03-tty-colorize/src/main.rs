#![allow(dead_code)]

enum AnsiEscapeCodes {
    Reset = 0,
    Bold = 1,
    Faint = 2,
    Italic = 3,
    Underline = 4,
    SlowBlink = 5,
    RapidBlink = 6,
    Strike = 9,
}

trait Bold {
    fn bold(&self) -> String;
}

impl Bold for String {
    fn bold(&self) -> String {
        format!("\x1b[{}m{}\x1b[0m", AnsiEscapeCodes::Bold as i8, self)
    }
}

trait Italic {
    fn italic(&self) -> String;
}

impl Italic for String {
    fn italic(&self) -> String {
        format!("\x1b[{}m{}\x1b[0m", AnsiEscapeCodes::Italic as i8, self)
    }
}

trait Blink {
    fn blink(&self, is_fast: bool) -> String;
}

impl Blink for String {
    fn blink(&self, is_fast: bool) -> String {
        let code = match is_fast {
            true => AnsiEscapeCodes::RapidBlink,
            false => AnsiEscapeCodes::SlowBlink,
        };

        format!("\x1b[{}m{}\x1b[0m", code as i8, self)
    }
}

trait Colorize {
    fn background(&self, rgb: u32) -> String;
    fn foreground(&self, rgb: u32) -> String;
}

fn extract(rgb: u32) -> (u8, u8, u8) {
    let r = (rgb >> 16) as u8;
    let g = ((rgb & 0xFF00) >> 8) as u8;
    let b = (rgb & 0xFF) as u8;

    (r, g, b)
}

impl Colorize for String {
    fn foreground(&self, rgb: u32) -> String {
        let (r, g, b) = extract(rgb);

        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, self)
    }

    fn background(&self, rgb: u32) -> String {
        let (r, g, b) = extract(rgb);

        format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, self)
    }
}

fn main() {
    let text = String::from("hello world");

    println!("{}", text.bold());
    println!("{}", text.italic());
    println!("{}", text.blink(true));
    println!("{}", text.blink(false));
    println!("{}", text.background(0xD6483B).foreground(0x161616));
}
