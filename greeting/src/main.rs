mod biology;
mod school; // 将 school 模块引入到 crate root 中

use biology::animal::cat::cat;
use biology::animal::dog::dog;

fn main() {
    let avg = school::teacher::average_students();
    println!("{}", avg);
    let cat_eat = cat::eat();
    println!("cat eat {}", cat_eat);
    let dog_eat = dog::eat();
    println!("dog eat {}", dog_eat);
}
