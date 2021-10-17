mod basic;
mod biology;
mod demo;
mod testbasic;

mod school; // 将 school 模块引入到 crate root 中

mod package;

use biology::animal::cat::cat;
use biology::animal::dog::dog;
use biology::people;
use demo::mydemo::testdemo;
use std::env;
use testbasic::basicmod::bmod;

// cargo run test sample.txt
fn main() {
    mytime();
    let args: Vec<String> = env::args().collect();
    println!("args len {}", args.len());
    if args.len() >= 3 {
        print!("{}\n", &args[0]);
        let query = &args[1];
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("In file {}", filename);
    }
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
    println!("{}", bfun());

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("condition met!")
}

fn bfun() -> usize {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);
    index
}
extern crate chrono; // extern 导入外部包
use chrono::{DateTime, FixedOffset, Local, Utc};
fn mytime() {
    let local_time = Local::now();
    println!(
        "时间格式化:{}",
        local_time.format("%Y-%m-%d %H:%M:%S").to_string()
    );

    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        utc_time.with_timezone(&china_timezone)
    );
    println!(
        "Time in Rio de Janeiro now is {}",
        utc_time.with_timezone(&rio_timezone)
    );
}
