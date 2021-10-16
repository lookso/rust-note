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
    mytime()
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

pub use chrono::{DateTime, Duration, Utc};

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}

fn mytime() {
    let now = Utc::now();
    println!("{}", now);

    let almost_three_weeks_from_now = now
        .checked_add_signed(Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
}
