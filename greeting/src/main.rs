mod testbasic;
mod biology;
mod demo;

mod school; // 将 school 模块引入到 crate root 中

use testbasic::basicmod::bmod;
use biology::animal::cat::cat;
use biology::animal::dog::dog;
use biology::people;
use demo::mydemo::testdemo;

fn main() {
    let avg = school::teacher::average_students();
    println!("{}", avg);
    let cat_eat = cat::eat();
    println!("cat eat {}", cat_eat);
    let dog_eat = dog::eat();
    println!("dog eat {}", dog_eat);

    let people_eat = people::man::eat();
    println!("people eat{}", people_eat);

    let out = testdemo::mytest();
    println!("mytest:{}", out);

    let fbnq = testdemo::fbnq(100);
    println!("fbnq:{}", fbnq);

    println!("{}", bmod::b());
    println!("{}",bfun());
}
fn bfun() ->usize{
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}",guess);
    index
}
