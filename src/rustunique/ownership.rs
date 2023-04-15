use crate::rustunique::rust_unique_features;
use crate::utils::terminal::finished_line;
use crate::utils::terminal::write_group;
use crate::utils::terminal::write_subject;
use crate::utils::terminal::write_subgroup;

pub fn rust_ownership_concept() {
    rust_unique_features();
    write_subject("Variable Scope");
    write_subgroup("string literal and String");
    basic_variable_scope();


    write_group("Ownership in Rust");
    write_subgroup("Move owner");
    rust_move_data_to_new_owner();

    write_subgroup("Clone owner");
    use_clone_to_do_a_deep_copy();

    write_subgroup("Ownership and function parameter");
    functions_and_ownership();

    write_subgroup("Ownership and function return");
    function_and_return_ownership();

    write_subgroup("Reference and Borrowing");
    refenrences_and_borrowing();

    write_subgroup("String slices");
    this_is_string_slices();
    string_slice_as_params();

    finished_line();
}

fn basic_variable_scope() {
    let ls = "Hello";
    let mut s = String::from("Hello");
    println!("literal string value :: {} || String value :: {}", ls, s);

    let ls = "Hello world!";
    s.push_str(", world!");
    println!("literal string value :: {} || String value :: {}", ls, s);
}

fn rust_move_data_to_new_owner() {
    let sirius =  String::from("I am the brightest star in the Milkeyway");
    println!("{}", sirius);

    println!("100 years later ....");
    let arcturus = sirius;
    //println!("{}", sirius);
    println!("{}", arcturus);
}

fn use_clone_to_do_a_deep_copy() {
    let jupiter = String::from("I am a giant gas planet");
    let satturn = jupiter.clone();

    println!("Hi, I am Jupiter, {}", jupiter);
    println!("Hi, I am Satturn, {}", satturn);
}

fn functions_and_ownership() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // s is no longer valid here

    let x = 10;
    makes_copy(x);
    println!("Number {} from x is still valid here", x);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("String {} is taken from the caller", some_string);
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("Number {} is copied from the caller", some_integer);
}

fn function_and_return_ownership() {
    let result = gives_ownership();
    println!("The result is {}", result);
}

fn gives_ownership() -> String { 
    let some_string = String::from("hello and hello"); 
    //some_string 
    return some_string;
}

fn refenrences_and_borrowing() {
    let star = String::from("The Solar system");
    let mut venus = String::from("Venus");

    println!("{}", get_planet_info(&star, &mut venus));
}

fn get_planet_info(star: &String, planet: &mut String) -> String {
    planet.push_str(" is a very nice planet in");
    std::format!("{} {}", planet, star)
}

fn this_is_string_slices() {
    let s = String::from("Hello Pluto");
    let first_word = &s[0..5];
    let second_word = &s[6..11];
    println!("from string {}, the 1st word is {} and the 2nd word is {}", s, first_word, second_word);
}

fn string_slice_as_params() {
    let normal_string = String::from("Hello Bangkok");
    let string_literal = "Hi Saimai";
    println!("The first word of {} is {}", normal_string, first_word(&normal_string[..]));
    println!("The first word of {} is {}", string_literal, first_word(&string_literal));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
        return &s[0..i];
        }
    }
    &s[..]
}
