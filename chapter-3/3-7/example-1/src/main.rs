use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

// 这段代码有数据竞争，可能打印 "a" 或 "b"，也可能什么都不打印
fn main() {
    let a = thread::spawn(|| {
        A.store(true, Ordering::SeqCst);
        if !B.load(Ordering::SeqCst) {
            unsafe {
                S.push_str("a");
            }
        }
    });

    let b = thread::spawn(|| {
        B.store(true, Ordering::SeqCst);
        if !A.load(Ordering::SeqCst) {
            unsafe {
                S.push_str("b");
            }
        }
    });

    a.join().unwrap();
    b.join().unwrap();

    println!("S: {}", unsafe { &S });
}
