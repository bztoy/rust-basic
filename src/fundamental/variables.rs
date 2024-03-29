use crate::fundamental::programming_fundamental_title;
use crate::utils::terminal::write_group;
use crate::utils::terminal::section_break;
use crate::utils::terminal::write_subject;
use crate::utils::terminal::finished_line;

pub fn basic_variables_in_rust() {
    programming_fundamental_title();
    
    variables_and_mutibility();
    shadowning_variable();

    finished_line();
}

fn variables_and_mutibility() {
    write_subject("Basic variables in Rust");

    let x = 5;
    print!("The value of immutable x is: {}\n", x);
    let mut y = 5;
    print!("The initial value of mutable y is: {}\n", y);
    y = 10;
    print!("The changed value of mutable y is: {}\n", y);

    section_break();
}

fn shadowning_variable() {
    write_group("Shadowing in Rust");

    let spaces = "@ .x. @";
    println!("Current value of spaces is {}", spaces);
    print_type_of(&spaces);

    let spaces = spaces.len();
    println!("Current value of spaces is {}", spaces);
    print_type_of(&spaces);

    section_break();
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
