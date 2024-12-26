use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    thread::spawn(move || {
        a.sort(); // cannot borrow data in an `Arc` as mutable
    }).join().unwrap();

    thread::spawn(move || {
        dbg!(b);
    }).join().unwrap();
}
