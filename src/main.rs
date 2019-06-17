extern crate rayon;
// use std::cell::Cell;
use std::ops::DerefMut;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use rayon::join;

fn main() {
    println!("Hello, world!");

    println!("Example zero output:");
    ex0();
    println!("Example one output:");
    ex1();
    println!("Example three output:");
    ex3();

    println!("done!");
}

fn ex0() {

    let (snd, rcv) = channel();
    join(move || {
        let mut v = Vec::new();
        v.push(0);
        let _ = snd.send(v);
        // v.push(1);
    },
         move || {
             let v = rcv.recv().unwrap();
             println!("Received: {:?}", v);
         });
}

fn ex1() {
    let mut v = Vec::new ();
    v.push (1);
    join (|| println !("Thread  1: {:?}", &v),
          || println !("Thread  2: {:?}", &v)
    );
    v.push (2);
}

// fn ex2() {
//     let mut c = Vec::new();
//     c.push(0);
//     join (
//         || c.push(1),
//         ||  println !("{:?}", c)
//     );
// }

fn ex3() {
    let mutex = Mutex::new(Vec::new());
    join (
        || {
            let mut guard = mutex.lock().unwrap();
            guard.deref_mut().push(0)
        },
        || {
            let mut guard = mutex.lock().unwrap();
            println!("{:?}", guard.deref_mut());
        }
    );
}
