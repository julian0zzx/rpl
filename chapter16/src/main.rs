
use std::thread;
use std::time::Duration;

use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    println!("Hello, world!");
    // test_spawn();
    // test_move();

    // test_pass_msg();

    // test_share_state();

}

// 16.1
fn test_spawn() {
    let handler = thread::spawn(move || {
        for i in 1 .. 10 {
            println!("number in spawned thread {}", i);
            thread::sleep(Duration::from_millis(20)); // 20ms
        }
    });
    // handler.join().unwrap();

    for i in 1 .. 5 {
        println!("number in test_spawn {}", i);
        thread::sleep(Duration::from_millis(30)); 
    }

    // handler.join().unwrap();
}

fn test_move() {
    let v = vec![1, 2, 3];
    let handler = thread::spawn(move || { // move !
        println!("spawned {:?}", v);
    });
    handler.join().unwrap();
}

// 16.2
fn test_pass_msg() {
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let ss = String::from("hi, world");
    //     tx.send(ss).unwrap();
    // });

    // let received = rx.recv().unwrap();
    // println!("received: {}", received);

    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let ns = vec![1, 3, 5];
        for i in ns {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(3));
        }
    });

    thread::spawn(move || {
        let ns = vec![2, 4, 6];
        for i2 in ns {
            tx2.send(i2).unwrap();
            thread::sleep(Duration::from_millis(3));
        }
    });

    for ri in rx {
        println!("received: {}", ri);
    }
}

// 16.3
fn test_share_state() {
    // let m = Mutex::new(5);
    // {
    //     let mut n = m.lock().unwrap();
    //     *n += 1;
    // }
    // println!("{:?}", m);

    let counter = Arc::new(Mutex::new(0)); // 
    let mut handles = vec![];

    for _ in 1 .. 10 {
        let ccc = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = ccc.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{:?}", *counter.lock().unwrap());
}
