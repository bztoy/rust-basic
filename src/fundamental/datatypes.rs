use crate::utils::terminal::write_group;

pub fn basic_date_types() {
    write_group("Basic Data Types in Rust");
    interger_types();
}

fn interger_types() {
    let temperature: i16 = -21;
    let wind_speed: u8 = 50;
    println!("[Today Weather] the temperture is {} and the wind speed is {}", temperature, wind_speed);
}
