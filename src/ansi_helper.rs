#![allow(dead_code)]

const CSI: &str = "\x1b[";

#[derive(Clone)]
pub struct RGB(u8, u8, u8);

/**
 * Default colours. https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit
 * A combination of VSCode and Windows 10 Console.
 */
pub mod colours {
    use super::RGB;

    pub const BLACK: RGB = RGB(0, 0, 0);
    pub const RED: RGB = RGB(197, 15, 31);
    pub const GREEN: RGB = RGB(19, 161, 14);
    pub const YELLOW: RGB = RGB(229, 229, 16);
    pub const BLUE: RGB = RGB(36, 114, 200);
    pub const MAGENTA: RGB = RGB(188, 63, 188);
    pub const CYAN: RGB = RGB(17, 168, 205);
    pub const WHITE: RGB = RGB(255, 255, 255);
    pub const GREY: RGB = RGB(102, 102, 102);
    pub const BRIGHT_RED: RGB = RGB(241, 76, 76);
    pub const BRIGHT_GREEN: RGB = RGB(22, 198, 12);
    pub const BRIGHT_YELLOW: RGB = RGB(245, 245, 67);
    pub const BRIGHT_BLUE: RGB = RGB(59, 120, 234);
    pub const BRIGHT_MAGENTA: RGB = RGB(214, 112, 214);
    pub const BRIGHT_CYAN: RGB = RGB(41, 184, 219);
}

pub fn reset_all() -> String {
    write_code("0m")
}

pub fn bold_on() -> String {
    write_code("1m")
}

pub fn bold_off() -> String {
    write_code("22m")
}

pub fn italics_on() -> String {
    write_code("3m")
}

pub fn italics_off() -> String {
    write_code("23m")
}

pub fn underline_on() -> String {
    write_code("4m")
}

pub fn underline_off() -> String {
    write_code("24m")
}

pub fn reset_fg_colour() -> String {
    write_code("39m")
}

pub fn reset_bg_colour() -> String {
    write_code("49m")
}

pub fn reset_colour() -> String {
    format!("{}39m{}49m", CSI, CSI)
}

pub fn set_fg_colour(c: &RGB) -> String {
    write_code(format!("38;2;{};{};{}m", c.0, c.1, c.2).as_str())
}

pub fn set_bg_colour(c: &RGB) -> String {
    write_code(format!("48;2;{};{};{}m", c.0, c.1, c.2).as_str())
}

pub fn move_up_lines(n: usize) -> String {
    write_code(format!("{}A", n).as_str())
}

fn write_code(code: &str) -> String {
    format!("{}{}", CSI, code)
}
