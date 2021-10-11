pub struct Person {
    name: String,
    address: String,
    age: i32,
}
impl Person {
    pub fn new(name: &str) -> Person {
        Person {
            name: String::from(name),
            address: String::from("chian beijing"),
            age: 10,
        }
    }
    /// 给一个友好的问候！
    /// 对被叫到的 `Person` 说 "Hello, [name]" 。
    pub fn hello(&self) {
        println!("Hello, {}!,From:{},Age:{}", self.name,self.address,self.age);
    }
}

#[test]
pub fn test_person() {
    let john = Person::new("John");
    john.hello();
}
