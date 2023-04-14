const ULINE_WIDGHT: u32 = 80;
const LINE: char = '-';
const UNDERLINE: char = '_';
const TOP_BAR: char = '=';
const GROUP: char = '+';

use std::str;

pub fn write_title(title: &str) {
    top();
    println!("{}", title);
    underline();
}

pub fn write_subject(subject: &str) {
    println!("{}", subject);
    underline();
}

pub fn write_group(text: &str) {
    group();
    println!("{}", text);
}

pub fn write_subgroup(text: &str) {
    uline();
    println!("{}", text);
    uline();
}

pub fn finished_line() {
    println!(" ");
    underline();
}

pub fn section_break() {
    uline();
}

fn top() {
    for _ in 0..ULINE_WIDGHT {
        print!("{}", TOP_BAR);
    }
    print!("\n");
}

fn group() {
    for _ in 0..ULINE_WIDGHT {
        print!("{}", GROUP);
    }
    print!("\n");
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
