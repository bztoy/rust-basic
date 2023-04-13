use regex::Regex;

pub mod fundamental;
pub mod utils;

use crate::fundamental::programming_fundamental;
use crate::fundamental::datatypes::basic_date_types;
use crate::fundamental::variables::basic_variables_in_rust;

struct Container {
    isx: bool,
    runner: fn(),
}

impl Container {
    fn run(&self){
        if self.isx {
            (self.runner)();
        }
    }
}

fn setup() -> Vec<Container> {
    let mut containers: Vec<Container> = Vec::new();
    containers.push(Container {
        isx: true,
        runner: programming_fundamental,
    });
    containers.push(Container {
        isx: true,
        runner: basic_variables_in_rust,
    });
    containers.push(Container {
        isx: false,
        runner: basic_date_types,
    });

    return containers;
}

fn rununit(containers: &Vec<Container>) {
    // loop through all items in the containers then
    // run each particular item that has isx is true
    for container in containers.iter() {
        container.run();
    }
}

fn main() {

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));

    let containers = setup();
    rununit(&containers);
}
