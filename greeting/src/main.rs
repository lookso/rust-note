// 保证 math 在 `./math.rs` 或 `./math/mod.rs` 中定义
mod math;
mod libs;

// 将两个符号带入范围，在 `math` 模块中保证都已导出
use math::{add, sub};
use libs::{my_main};

fn main() {
    let random_boolean = rand::random();
    println!("You {}!", if random_boolean { "win" } else { "lose" });

    let add_res = add(1, 2);
    println!("1 + 2 = {}", add_res);
    let sub_res = sub(10, 2);
    println!("10 * 2 = {}", sub_res);
    my_main();
}