use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    X.store(1, Ordering::Relaxed);
    let t = std::thread::spawn(f);
    X.store(2, Ordering::Relaxed);
    t.join().unwrap();
    X.store(3, Ordering::Relaxed);
    println!("Hello, world!");
}

fn f() {
    let x = X.load(Ordering::Relaxed);
    assert!(x == 1 || x == 2);
}
