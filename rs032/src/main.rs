#![feature(start)] // rustup default nightly

use std::thread;
use std::sync::mpsc::{ self };
use std::time::Duration;

// use std::rc::Rc; // single thread works.
use std::sync::{Arc, Mutex};

#[start] // #[main] removed https://github.com/rust-lang/rust/pull/93753 https://stackoverflow.com/questions/71024443/what-is-the-built-in-main-attribute
fn my_main(_argc: isize, _argv: *const *const u8) -> isize {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();
    
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    let v = vec![1,2,3];

    // let handle = thread::spawn(|| { // Error: closure borrow v not allowed.
    let handle = thread::spawn( move || -> Vec<i32> {
        println!("Here's a vector: {:?}", v);
        v
    });

    // drop(v); // Error: v is moved to closure.

    match handle.join() {
        Ok(result) => {
            println!("Thread Result: {:?}", result);
        },
        Err(error) => {
            println!("Thread Error: {:?}", error);
        },
    };

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn( move || {
        // tx1.send(1) // must be the same type
        tx1.send("t: T".to_owned())
    });
    let handle = thread::spawn( move || {
        let msg = String::from("hi");
        match tx.send(msg) {
            Ok(_) => {
                println!("success!");
            },
            Err(error) => {
                println!("{}", error);
            },
        };
        // println!("{}", msg); // send move msg to another thread

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    let msg = rx.recv().unwrap();
    println!("{} from channel.", msg);

    for received in rx {
        println!("Got: {}", received);
    }

    handle.join().unwrap();

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // let counter = Mutex::new(0);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Rc::clone(&counter); // not a single thread
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    0
}
