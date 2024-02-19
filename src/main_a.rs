macro_rules! create_function {
    // 此宏接受一个 `ident` 指示符表示的参数，并创建一个名为 `$func_name` 的函数。
    // `ident` 指示符用于变量名或函数名
    ($func_name:ident) => {
        fn $func_name() {
            // `stringify!` 宏把 `ident` 转换成字符串。
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}
// block
// expr 用于表达式
// ident 用于变量名或函数名
// item
// pat (模式 pattern)
// path
// stmt (语句 statement)
// tt (标记树 token tree)
// ty (类型 type)

// 借助上述宏来创建名为 `foo` 和 `bar` 的函数。
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // 此宏接受一个 `expr` 类型的表达式，并将它作为字符串，连同其结果一起
    // 打印出来。
    // `expr` 指示符表示表达式。
    ($expression:expr) => {
        // `stringify!` 把表达式*原样*转换成一个字符串。
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

mod example;

use example::hashmapdemo;
use example::basic;
use example::jsondemo;
extern crate json;
extern crate mysql;
extern crate package_utils;
use basic::Summary;

// 教程:https://course.rs/basic/variable.html
fn main_a() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let v = Arc::new(Mutex::new(vec![1, 2, 3]));

    let v_clone = Arc::clone(&v);
    let handle = thread::spawn(move || {
        let v = v_clone.lock().unwrap();
        println!("Here's a vector: {:?}", v);
    });

    // 主线程中打印v
    {
        let v = v.lock().unwrap();
        println!("v:{:#?}", v);
    }

    handle.join().unwrap();


    demo_try();
    package_utils::sync_channel();
    hashmapdemo::hashmap_fun();
    jsondemo::json_func();

    foo();
    bar();

    print_result!(1u32 + 1);

    // 回想一下，代码块也是表达式！
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    let post = basic::Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = basic::Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());

}

fn demo_try() {
    assert!(0.1_64 + 0.2_64 -0.3_64>0.0001); // 浮点数精度问题,不能直接判断是否相等
    println!("浮点数相减:{}",format!("{:.2}", 56.1 / 32.2));
    let one_million: f64 = 1_000_000.123;
    println!("{:.4}", one_million.ceil());
    let mut t:i32 = 1;
    let tt:i64 = t as i64; // as 类型转换
    let st:String=t.to_string();
    let b:bool = t==tt as i32;
    {
        t=t+1;
        println!("t:{}",t);
    }
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    let a:char='A';
    let unicode=a as u8;
    let mut hello   = "中国人".to_string()+"nb";
    hello.push_str("确实nb");
    hello.push('1');
    hello.insert_str(6, " I like");
    println!("hello:{}", hello);

    let s = &hello[0..3];
    println!("s:{}", s);

//  在Rust中，{}和{:?}是println!宏和format!宏中用于格式化输出的占位符。
// {}：这是默认的占位符，用于打印实现了std::fmt::Display trait的值。Display trait用于提供用户友好的输出。
// {:?}：这个占位符用于打印实现了std::fmt::Debug trait的值。Debug trait用于提供调试信息，它的输出通常包含更多的详细信息，而且可能对用户来说不那么友好。
// 例如，对于一个复杂的结构体，Display可能只打印出一些关键的信息，而Debug会打印出所有的字段和它们的值。

    print!("t:{},tt:{},st:{},b:{},a:{}\n", t.abs(),tt,st,b,unicode);
    println!("tup:{:?}",demo_tup());

    println!("struct:{:#?}",demo_struct("123".to_string(),"456".to_string()));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five:{:?},six:{:?},none:{:?}",five,six,none);
    demo_enum();

    let mut people = Human::new(30, "toms".to_string());
    people.set_name("jack");
    let new_name = people.get_name().to_string();
    println!("people.hight:{},new_name:{}", people.hight(),new_name);

    println!("add i8: {}", basic::add(2i8, 3i8));
}


fn demo_tup()->(i32,f64,String){
    let tup = (500, 6.4, 1);
    let  new_tup:(i32,f64,String)=(tup.0,tup.1,tup.2.to_string());
    return new_tup;
}
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn demo_struct(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
 }

 #[allow(dead_code)]
 enum Message {
     Quit(i32),
     Move { x: i32, y: i32 },
     Write(String),
     ChangeColor(Color),
 }
 
 fn demo_enum() {
     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
 
     match msg {
         Message::ChangeColor(Color::Rgb(r, g, b)) => {
             println!(
                 "Change the color to red {}, green {}, and blue {}",
                 r,
                 g,
                 b
             )
         }
         Message::ChangeColor(Color::Hsv(h, s, v)) => {
             println!(
                 "Change the color to hue {}, saturation {}, and value {}",
                 h,
                 s,
                 v
             )
         }
         _ => ()
     }
 }
 #[allow(dead_code)]
 pub struct Human {
    hight: u32,
    name: String,
}

impl Human {
    pub fn new(hight: u32, name: String) -> Self {
        Human { hight, name }
    }
    // pub 可以被其他mod访问到
    pub fn hight(&self) -> u32 {
        return self.hight;
    }
    pub fn set_name(&mut self,new_name:&str) {
        self.name=new_name.to_string()
    }
    pub fn get_name(&mut self) -> String {
        self.name.to_string()
    }
}
