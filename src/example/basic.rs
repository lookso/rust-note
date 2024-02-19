use std::ops::Add;

pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
// 类似interface{}
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}


fn process_int(_input: u32) {}
fn process(_s: String) {}
fn basic_main() {
    let n = 1_u32; // 1_u32 copy 只适合简单的数字类型
    process_int(n); // Ownership of the number in `n` copied into `process`
    process_int(n); // `n` can be used again because it wasn't moved, it was copied.

    let s = String::from("Hello, world!");
    process(s.clone()); // Passing another value, cloned from `s`.
    process(s); // s was never moved and so it can still be used.
    life_time();
}

struct MyBox<T> {
    a: T,
}
use ::std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.a
    }
}
fn fbnq(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    fbnq(n - 1) + fbnq(n - 2)
}
fn life_time() {
    use std::fs::File;
    println!("Hello World!");
    let mut f = 0.1;
    for i in 1..100 {
        f = f + 0.1;
        println!("{},{:?}", i, f)
    }
    let f = File::open("hello.txt");
    if let Ok(file) = f {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }
    println!("fbnq:{}", fbnq(20));

    let b = MyBox { a: 10 };
    print!("{}", *(b.deref()));

    tarit_demo();
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");
    let result;
    {
        result = longest_word(&magic1, &magic2);
    }
    println!("The longest magic word is {}", result);
    let n1 = String::from("hello");
    // let n2 = n1.clone();
    let n2 = &n1; // 借用
    format!("{},{}", n1, n2);

    let n3 = if n1 == *n2 {
        14;
    } else {
        12;
    };
    println!("{},{:?}", n1, n3);
}

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
trait Man {
    fn man(&self) -> i32;
}
struct Human {
    name: String,
    age: i32,
}
impl Man for Human {
    fn man(&self) -> i32 {
        self.age
    }
}
fn tarit_demo() {
    let h = Human {
        name: "123".to_string(),
        age: 123,
    };
    println!("{},{}", h.name, h.age);
    h.man();
    firstexample();
}
fn firstexample() {
    println!("Hello World!");
    let mut f = 0.1;
    for i in 1..100 {
        f = f + 0.1;
        println!("{},{}", i, f);
    }
    println!("fbnq:{}", fbnq(20));
    let hm = Family {
        age: 120,
        _name: String::from("jack"),
    };
    println!("{}", hm.woman());
}
struct Family {
    age: i32,
    _name: String,
}
trait Person {
    fn woman(&self) -> i32;
}
impl Person for Family {
    fn woman(&self) -> i32 {
        self.age
    }
}
