mod biology;
mod demo;

mod school; // 将 school 模块引入到 crate root 中

use biology::animal::cat::cat;
use biology::animal::dog::dog;
use demo::mydemo::testdemo;

fn main() {
    let avg = school::teacher::average_students();
    println!("{}", avg);
    let cat_eat = cat::eat();
    println!("cat eat {}", cat_eat);
    let dog_eat = dog::eat();
    println!("dog eat {}", dog_eat);

    use biology::people;
    let people_eat = people::man::eat();
    println!("people eat{}", people_eat);

    let out = testdemo::mytest();
    println!("mytest:{}", out);
}
