#[derive(Debug)]
struct Person<'a> {
    name: &'a str, // 'a生命周期,Person生命周期不能大于a
    age: u8,
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Subtract, // 变量如果没有使用就以_ 开头
    Add,
}
// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

#[allow(unused)]
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
// 全局变量是在在所有其他作用域之外声明的。
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10; // const 定义 不可修改

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    // 类型转换
    let decimal = 65.4321_f32;
    // 可以显式转换
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    let my_str = "hello";
    let my_string = String::from(my_str);

    let num = Number::from(30);
    println!("My number is {:?},{}", num, my_string);

    // 类型推断
    // 因为有类型说明，编译器知道 `elem` 的类型是 u8。
    let elem = 5u8;

    // 创建一个空向量（vector，即不定长的，可以增长的数组）。
    let mut vec = Vec::new();
    // 现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<_>`）

    // 在向量中插入 `elem`。
    vec.push(elem);
    // 啊哈！现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）。
    // 试一试 ^ 注释掉 `vec.push(elem)` 这一行。
    println!("{:?}", vec);

    // const
    let n = 16;
    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    let _x = Operations::Add;

    let very = VeryVerboseEnumOfThingsToDoWithNumbers::run(&Operations::Subtract, 100, 20);

    for n in 1..10 {
        println!("{}", n)
    }
    let mut names = vec!["Bob", "Frank", "Ferris"];
    names.sort();

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            n @ "Bob" => println!("this is {}", n), // 通过@指定到变量里
            // 处理其他情况
            _ => println!("Hello {}", name),
        }
    }

    // 美化打印
    println!("{:#?}\nvery:{}", peter, very);
}
