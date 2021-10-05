mod testmodule {
    pub enum Person {
        King { name: String },
        _Quene,
    }
}

fn main() {
    let person = testmodule::Person::King {
        name: String::from("Blue"),
    };
    match person {
        testmodule::Person::King { name } => {
            println!("{}", name);
        }
        _ => {}
    }
}
