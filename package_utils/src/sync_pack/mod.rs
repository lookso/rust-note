pub mod lock;

extern crate parking_lot;

use parking_lot::Mutex;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

pub(crate) fn sync_test() {
    const N: usize = 10;
    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();
    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        let child = thread::spawn(move || {
            let mut data = data.lock();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
        });
        child.join().unwrap();
    }

    println!("{:?}", rx.recv().unwrap());
}
