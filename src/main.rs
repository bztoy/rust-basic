use regex::Regex;

pub mod fundamental;
pub mod utils;
pub mod rustunique;
pub mod rustfeatures;

use crate::fundamental::datatypes::basic_scala_data_types;
use crate::fundamental::datatypes::basic_compound_data_types;
use crate::fundamental::variables::basic_variables_in_rust;
use crate::fundamental::functions::basic_functions_in_rust;
use crate::fundamental::controlflow::control_flow_in_rust;
use crate::rustunique::ownership::rust_ownership_concept;

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

    // Common Programming Concepts
    containers.push(Container {
        isx: false,
        runner: basic_variables_in_rust,
    });
    containers.push(Container {
        isx: false,
        runner: basic_scala_data_types,
    });
    containers.push(Container {
        isx: false,
        runner: basic_compound_data_types,
    });
    containers.push(Container {
        isx: false,
        runner: basic_functions_in_rust,
    });
    containers.push(Container {
        isx: false,
        runner: control_flow_in_rust,
    });

    // Understanding Owership
    containers.push(Container {
        isx: true,
        runner: rust_ownership_concept,
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
