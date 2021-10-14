#![allow(unused)]
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_cond() {
    let pair = Arc::new((Mutex::new(4), Condvar::new()));
    let pair2 = pair.clone();

    let thread1 = thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        for i in 0..4 {
            thread::sleep(Duration::from_secs(1));
            let mut started = lock.lock().unwrap();
            *started -= 1;
            println!("...");
            cvar.notify_one();
            println!("child thread - value:{}", *started);
        }
    });

    let &(ref lock, ref cvar) = &*pair;
    {
        let mut started = lock.lock().unwrap();
        println!("before wait {}", *started);
        while *started > 2 {
            println!("-");
            started = cvar.wait(started).unwrap();
            println!("+");
        }
        println!("after wait {}", *started);
    }
    thread1.join();
}
