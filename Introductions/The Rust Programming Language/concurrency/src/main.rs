// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/concurrency.html
#![allow(unused_mut, unused_variables)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Concurrency


    ////////////////////////////////////////////////////////////////////////////////
    // Threads

    thread::spawn(|| {
        println!("Hello from a thread!");
    });

    let handle = thread::spawn(|| {
        "Hello from a thread!"
    });

    println!("{}", handle.join().unwrap());


    ////////////////////////////////////////////////////////////////////////////////
    // Safe Shared Mutable State

    // First

    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        thread::spawn(move || {
            // data[0] += i; // error[E0382]: capture of moved value: `data`
        });
    }

    thread::sleep(Duration::from_millis(50));

    // Arc

    let mut data = Arc::new(vec![1, 2, 3]);

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            // data[i] += 1; // error: cannot borrow immutable borrowed content as mutable
        });
    }

    thread::sleep(Duration::from_millis(50));

    // Mutex

    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }

    thread::sleep(Duration::from_millis(50));


    ////////////////////////////////////////////////////////////////////////////////
    // Channels

    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }


    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let tx = tx.clone();

        thread::spawn(move || {
            let answer = i * i;

            tx.send(answer).unwrap();
        });
    }

    for _ in 0..10 {
        println!("{}", rx.recv().unwrap());
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Panics

    let handle = thread::spawn(move || {
        panic!("oops!");
    });

    let result = handle.join();

    assert!(result.is_err());
}
