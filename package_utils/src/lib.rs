#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
mod sync_pack;
use sync_pack::lock::call as lcokCall;

pub fn sync_channel() {
    lcokCall();
    sync_pack::sync_test();
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx); //同一通道，增加一个发送者

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        //一个通道一个接收者，接收两个线程的两个发送者
        println!("Got: {}", received);
    }

    // --snip--
    println!("---------------");
}
