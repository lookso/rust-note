#[derive(Debug)]
pub struct Person {
    name: String,
    address: String,
    age: i32,
}
impl Person {
    #[allow(unused)]
    pub fn new(name: &str) -> Person {
        Person {
            name: String::from(name),
            address: String::from("chian beijing"),
            age: 10,
        }
    }
    /// 给一个友好的问候！
    /// 对被叫到的 `Person` 说 "Hello, [name]" 。
    // `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
    // 编译器提供了 dead_code（死代码，无效代码）lint，这会对未使用的函数 产生警告。可以用一个属性来禁用这个 lint。
    #[allow(dead_code)]
    pub fn hello(&self) {
        println!(
            "Hello, {}!,From:{},Age:{}",
            self.name, self.address, self.age
        );
    }
}

#[test]
#[allow(unused)]
pub fn test_person() {
    let john = Person::new("John");
    println!("{:?}", john);
    john.hello();
}

#[test]
#[allow(unused)]
pub fn test_thread() {
    use std::thread;
    use std::time::Duration;

    let tuple_name: (i32, i32) = (1, 20);
    println!("tuple_list:{:?},{}", tuple_name, tuple_name.0);

    let n = |x: i32, y: i32| -> (i32, i32) { (x + y, x * y) };
    println!("{:?}", n(2, 4));

    // 匿名函数 |data:type|->type{}
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 11..15 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
