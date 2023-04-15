use crate::fundamental::programming_fundamental_title;
use crate::utils::terminal::write_subject;
use crate::utils::terminal::write_subgroup;
use crate::utils::terminal::write_group;
use crate::utils::terminal::finished_line;

pub fn basic_functions_in_rust() {
    programming_fundamental_title();
    write_subject("Function in Rust");

    write_group("Parameters, Arguments, Expression");
    write_subgroup("Parameters and return");
    println!("{}", create_rider_info("Alex Rins", 42));
    println!("{}", create_rider_info("Alex Marquez", 73));
    write_subgroup("Expression");
    function_expression();

    finished_line();
}

fn create_rider_info(name: &str, number: u8) -> String {
    return format!("Rider name is {}. Rider number is {}.",name, number);
}

fn function_expression() {
    let y = {
        let x = 3;
        // below statement does not include ending ; the compiler will treat this line as an expression;
        x + 3
    };
    println!("The value of y is: {}", y);
}
