const ULINE_WIDGHT: u32 = 80;
const LINE: char = '-';
const UNDERLINE: char = '_';

use std::str;

pub fn write_subject(header: &str) {
    underline();
    uline();
    println!("{}", header);
    uline();
}

pub fn write_group(group: &str) {
    println!("{}", group);
    uline();
}

pub fn section_break() {
    uline();
}

fn uline() {
    for _ in 0..ULINE_WIDGHT {
        print!("{}", LINE);
    }
    print!("\n");
}
fn underline() {
    for _ in 0..ULINE_WIDGHT {
        print!("{}", UNDERLINE);
    }
    print!("\n");
}
