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

// 在测试函数前面加test_ 不是必须的，却是个好习惯。
#[test]
fn test_say_hello() {
    print!("我只是个测试了，你想怎样");
}
