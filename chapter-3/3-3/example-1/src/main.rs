use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    X.store(1, Ordering::Relaxed);
    let t = std::thread::spawn(f);
    X.store(2, Ordering::Relaxed);
    t.join().unwrap();
    X.store(3, Ordering::Relaxed);
}

fn f() {
    let x = X.load(Ordering::Relaxed);
    println!("x: {}", x);
    assert!(x == 1 || x == 2); // 断言：变量x的值必须等于1或者等于2。
}
