macro_rules! create_function {
    // 此宏接受一个 `ident` 指示符表示的参数，并创建一个名为 `$func_name` 的函数。
    // `ident` 指示符用于变量名或函数名
    ($func_name:ident) => {
        fn $func_name() {
            // `stringify!` 宏把 `ident` 转换成字符串。
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}
// block
// expr 用于表达式
// ident 用于变量名或函数名
// item
// pat (模式 pattern)
// path
// stmt (语句 statement)
// tt (标记树 token tree)
// ty (类型 type)

// 借助上述宏来创建名为 `foo` 和 `bar` 的函数。
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // 此宏接受一个 `expr` 类型的表达式，并将它作为字符串，连同其结果一起
    // 打印出来。
    // `expr` 指示符表示表达式。
    ($expression:expr) => {
        // `stringify!` 把表达式*原样*转换成一个字符串。
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

mod example;
use example::hashmapdemo;
use example::jsondemo;
extern crate json;
extern crate mysql;
extern crate package_utils;

fn main() {
    package_utils::sync_channel();
    hashmapdemo::hashmap_fun();
    jsondemo::json_func();

    foo();
    bar();

    print_result!(1u32 + 1);

    // 回想一下，代码块也是表达式！
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
