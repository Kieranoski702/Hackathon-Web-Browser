const CSI: &str = "\x1b[";

pub fn bold_on() {
    write_code("1m");
}

pub fn bold_off() {
    write_code("22m");
}

pub fn italics_on() {
    write_code("3m");
}

pub fn italics_off() {
    write_code("23m");
}

fn write_code(code: &str) {
    print!("{}{}", CSI, code);
}
