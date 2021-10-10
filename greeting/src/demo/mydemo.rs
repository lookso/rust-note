pub mod testdemo {
    pub enum Person {
        King { name: String },
        _Quene,
    }
    pub fn mytest() -> String {
        let person = Person::King {
            name: String::from("Blue"),
        };
        match person {
            Person::King { name } => {
                println!("{}", name);
            }
            _ => {}
        }
        format!("{}", "haha")
    }
    pub fn fbnq(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        fbnq(n - 1) + fbnq(n - 2)
    }
}
