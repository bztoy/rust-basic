use crate::fundamental::programming_fundamental_title;
use crate::utils::terminal::finished_line;
use crate::utils::terminal::write_group;
use crate::utils::terminal::write_subgroup;
use crate::utils::terminal::write_subject;

pub fn control_flow_in_rust() {
    programming_fundamental_title();
    write_subject("Control Flow in Rust");

    write_group("IF Expression, Branches and Loop");

    write_subgroup("If else branches");
    if_else_basic_braches();

    write_subgroup("Using if in a let statement");
    using_if_in_a_let_statement();

    write_subgroup("Repetition with Loops");
    for_loop_basic();
    while_loop_basic();

    write_subgroup("Looping Through a Collection with for");
    loop_through_a_collection();

    finished_line();
}

fn if_else_basic_braches() {
    let number = 7;

    if number % 3 == 0 {
        println!("{} has 3 as a factor!", number);
    } else if number % 5 == 0 {
        println!("{} has 5 as a factor!", number);
    } else {
        println!("{} is a magic number!", number);
    }
}

fn using_if_in_a_let_statement() {
    let set: [u8; 2] = [4, 2];
    let answer: &str = if set[0] % set[1] == 0 {
        "Even"
    } else {
        "Odd"
    };

    println!("The result of {:?} is {} number", set, answer);
}

fn for_loop_basic() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Counter is {} and result is {}", counter, result);
}

fn while_loop_basic() {
    let mut number = 5;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

fn loop_through_a_collection() {
    let a = [10, 20, 30, 40, 50, 60];

    for element in a {
        println!("element: {}!", element);
    }
}
