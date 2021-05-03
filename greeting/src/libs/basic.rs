use std::io;

// #[warn(dead_code)]
// #[warn(unused_imports)]
#[allow(dead_code)]
mod nation {
    pub mod government {
        pub fn govern() {
            println!("this function()")
        }
    }

    mod congress {
        pub fn legislate() {}
    }

    mod court {
        fn judicial() {
            super::congress::legislate();
        }
    }
}

pub fn my_main() {
    nation::government::govern();
    let fruits = ["mango", "apple", "banana", "litchi", "watermelon"];
    for f in fruits.iter() {
        print!("{} ", f);
    }
    println!("Hello, world!");
    let a = 123;
    println!("{}", a);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{},{}", x, y);

    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0 等于 500
    // tup.1 等于 6.4
    // tup.2 等于 1
    //let (x, y, z) = tup;
    // y 等于 6.4
    //println!("{}{}{}",x,y,z)
    arr_function();
    println!("{}", add(1, 2));

    let mut s = 12; // 声明变量可变,就要改变他
    println!("{}", s);
    s = 234;
    println!("{}", s);

    println!("{}", s);
    println!("{}", xh_function());
    m_function();
    basic();
}

fn xh_function() -> i32 {
    println!("-----循环------");
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }
    let a = [10, 20, 30, 40, 50];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
    return number;
}

fn add(a: i32, b: i32) -> i32 {
    let y = {
        let x = 3;
        x + 1
    };
    println!("-------------");
    println!("y:{}", y);
    return a + b;
}

fn arr_function() {
    let aa = [1, 2, 3, 4, 5];
    // a 是一个长度为 5 的整型数组

    let bb = ["January", "February", "March"];
    // b 是一个长度为 3 的字符串数组
    println!("{}", bb[0]);

    let c: [i32; 5] = [1, 2, 3, 4, 5];
    // c 是一个长度为 5 的 i32 数组
    println!("{}", c[0]);
    let d = [3; 5];
    // 等同于 let d = [3, 3, 3, 3, 3];
    println!("{}", d[0]);

    let first = aa[0];
    let second = aa[1];
    // 数组访问
    println!("{}{}", first, second);
    //aa[0] = 123; // 错误：数组 a 不可变
    let mut aa = [1, 2, 3];
    aa[0] = 4; // 正确
}

fn m_function() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

fn basic() {
    println!("---basic----");
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);
    slice_func();
}

struct Site {
    domain: String,
    name: String,
    width: u32,
    height: u32,
}

impl Site {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn slice_func() {
    let mut s = String::from("runoob");
    s.push_str("hello");
    //let slice = &s[0..3];
    //s.push_str("yes!"); // 错误,s 被部分引用，禁止更改其值。
    println!("slice = {}", s);

    let run_oob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        width: 30,
        height: 50,
    };
    println!("{},{},{}", run_oob.area(), run_oob.domain, run_oob.name);

    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(10, 12, 123);
    let origin = Point(0.0, 0.0);
    println!("{}", black.0);
    print!("{}", origin.1);

    // 枚举
    enum Book {
        Papery { index: u32 },
        Electronic { url: String },
    }

    let book = Book::Papery { index: 1001 };
    let _ebook = Book::Electronic { url: String::from("url...") };

    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        }
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
}