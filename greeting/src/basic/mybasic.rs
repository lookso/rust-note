#[derive(Debug)]
struct Person<'a> {
    name: &'a str, // 'a生命周期,Person生命周期不能大于a
    age: u8,
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    _Subtract, // 变量不适用以_ 开头
    Add,
}
// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

#[allow(unused)]
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::_Subtract => x - y,
        }
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    let _x = Operations::Add;

    // 美化打印
    println!("{:#?}", peter);
}
