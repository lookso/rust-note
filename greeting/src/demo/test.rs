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

// 泛型介绍

#[test]
#[allow(unused)]
pub fn test_foo() {
    // 一个具体类型 `A`。
    struct A;

    // 在定义类型 `Single` 时，第一次使用类型 `A` 之前没有写 `<A>`。
    // 因此，`Single` 是个具体类型，`A` 取上面的定义。
    struct Single(A);
    // ^ 这里是 `Single` 对类型 `A` 的第一次使用。
    // 此处 `<T>` 在第一次使用 `T` 前出现，所以 `SingleGen` 是一个泛型类型。
    // 因为 `T` 是泛型的，所以它可以是任何类型，包括在上面定义的具体类型 `A`。
    struct SingleGen<T>(T);

    // `Single` 是具体类型，并且显式地使用类型 `A`。
    let _s = Single(A);

    // 创建一个 `SingleGen<char>` 类型的变量 `_char`，并令其值为 `SingleGen('a')`
    // 这里的 `SingleGen` 的类型参数是显式指定的。
    let _char: SingleGen<char> = SingleGen('a');
    // `SingleGen` 的类型参数也可以隐式地指定。
    let _t = SingleGen(A); // 使用在上面定义的 `A`。
    let _i32 = SingleGen(6); // 使用 `i32` 类型。
    let _char = SingleGen('a'); // 使用 `char`。
}
