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
#[warn(unreachable_code)]
fn test_say_hello() {
    print!("我只是个测试了，你想怎样\n");

    let mut my_str = String::new();
    my_str.push_str("my_str1");
    my_str.push('m');
    let mut str_test: String = {
        let mut mascot = String::from("ferris");
        // transfer ownership of mascot to the variable ferris.
        // let ferris = mascot;
        // ferris dropped here. The string data memory will be freed here.
        //my_str.push_str(&ferris);
        mascot.push_str(&'a'.to_string());
        return mascot.push_str("123");
    };
    let mut new_str = str_test.push_str(r#"my_str_add_test"#);
    println!("{:?}", new_str);
}
