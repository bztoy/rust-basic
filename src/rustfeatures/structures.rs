use crate::rustfeatures::rust_key_features_title;
use crate::utils::terminal::finished_line;
use crate::utils::terminal::write_group;
use crate::utils::terminal::write_subject;
use crate::utils::terminal::write_subgroup;

struct MotoGPBike {
    model: String,
    engine_capacity: u32,
    approved_by_fim: bool,
}

#[derive(Debug)]
struct Gearbox {
    ratio: u32,
    speed: u8,
    gearbox_type: String,
}

pub fn basic_structure_funtionality() {
    rust_key_features_title();
    write_subject("Structures");
    write_group("Structs");


    write_subgroup("Defining and Instantiating Structs");
    motogp_bike_struct_example();
    write_subgroup("Derived Traits");
    how_derived_traits_work();

    write_subgroup("Method");
    struct_and_method();

    finished_line();
}

fn build_motogp_bike(model: String, engine_capacity: u32) -> MotoGPBike {
    // model and engine_capacity will be assigned autometically by short hand init
    MotoGPBike { model, engine_capacity, approved_by_fim: true }
}

fn show_motogp_bike_info(bike: &MotoGPBike) {
    println!("Bike name: {}, Engine capacity: {}, This bike has approved by FIM: {}", bike.model, bike.engine_capacity, bike.approved_by_fim);
}

fn motogp_bike_struct_example() {
    let gsxrr = build_motogp_bike(String::from("GSX-RR"), 1000);
    show_motogp_bike_info(&gsxrr);

    // Struct Update Syntax
    let rc213v = MotoGPBike {
        model: String::from("RC213V"),
        ..gsxrr
    };
    show_motogp_bike_info(&rc213v);
}

fn how_derived_traits_work() {
    let normal_gearbox = Gearbox {
        ratio: 130,
        speed: 6,
        gearbox_type: String::from("normal"),
    };
    println!("This is a {} gearbox, it comes with {} speed and {} changing ratio", normal_gearbox.gearbox_type, normal_gearbox.speed, normal_gearbox.ratio);
    println!("This is the component of the gearbox: {:?}", normal_gearbox);

}

struct Book {
    title: String,
    pages: u32,
    width: u32,
    height: u32,
}

impl Book {
    fn dimension(&self) -> u32 {
        self.width * self.height
    }
    fn info(&self) {
        println!("the book {} has total {} pages, it requires {} squire centemeters to keep", self.title, self.pages, self.dimension());
    }

    // mutably self
    fn increse_size(&self) {
        println!("the new size will be {} width and {} height", self.width + 1, self.height + 1);
    }

    // this is a associated function
    fn new_book(title: String, pages: u32, width: u32, height: u32) -> Book {
        Book { title , pages, width, height }
    }
}

fn struct_and_method() {
    let gangsaaa = Book {
        title: String::from("Gangzaa adventure"),
        pages: 120,
        width: 5,
        height: 4,
    };

    let naruto = Book::new_book(String::from("Naruto"), 80, 4, 4);

    gangsaaa.info();
    gangsaaa.increse_size();
    naruto.info();

}
