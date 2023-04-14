use crate::fundamental::programming_fundamental_title;
use crate::utils::terminal::finished_line;
use crate::utils::terminal::write_group;
use crate::utils::terminal::write_subgroup;
use crate::utils::terminal::write_subject;

pub fn basic_scala_data_types() {
    programming_fundamental_title();
    write_subject("Basic data types in Rust");

    write_group("Scalar Types");

    write_subgroup("Integers");
    interger_types();
    integer_literal_in_rust();

    write_subgroup("Floating-Points");
    floating_point_types();

    write_subgroup("Boolean types");
    boolean_types();

    finished_line();
}

fn interger_types() {
    let temperature: i16 = -21;
    let wind_speed: u8 = 50;
    println!("[Today Weather] the temperture is {} and the wind speed is {}", temperature, wind_speed);
}

fn integer_literal_in_rust() {
    let score_decimal: u32 = 98_222;
    let score_hex: u8 = 0xff;
    let score_octal: u8 = 0o77;
    let score_binary: u32 = 0b1111_0000;
    let score_byte: u8 = b'A';
    println!("[Sample of Integer Literals] Decimal:: {} Hex:: {} Octal:: {} Binary:: {} Byte:: {}", score_decimal, score_hex, score_octal, score_binary, score_byte);
}

fn floating_point_types() {
    let everage_speed = 315.8;
    let top_speed: f32 = 370.5;
    println!("[Suzuki GSX-RR] Top speed is {} ... everage speed is {}", everage_speed, top_speed);
}

fn boolean_types() {
    let is_sunday_weekend = true;
    let is_the_next_day_is_holiday: bool = false;
    println!("[Boolean fun fact] Is Sunday is weekend :: {} ... Is the next day is holiday :: {}", is_sunday_weekend, is_the_next_day_is_holiday);
}

pub fn basic_compound_data_types() {
    programming_fundamental_title();
    write_subject("Basic compoud types in Rust");

    write_group("Tuple and Array");

    write_subgroup("Tuple");
    tuple_type();
    
    write_subgroup("Array");
    array_type();
}

fn tuple_type() {
    let sample_tuple = (500, 1.1, 7, true);
    let (w, x, y, z) = sample_tuple;
    println!("The values of tuple members are {} :: {} :: {} :: {}", w, x, y, z);

    let mm93: (&str, u8) = ("Marc Marquez", 93);
    println!("Rider name: {}, Racing number: {}", mm93.0, mm93.1);
}

fn array_type() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    for month in months {
        print!("{}, ", month);
    }
    print!("\n");

    let a: [i16; 5] = [1, 2, 3, 4, 5];
    println!("Array value is {:?} and The length of this array is {}",a, a.len());

    // a more concise way to assign initial value to an array
    let array_with_initial_value = [9; 4];
    println!("Array with initial value {:?} and has {} as the length of the array",array_with_initial_value, array_with_initial_value.len());
}
