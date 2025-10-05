
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::thread;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(123, Ordering::Relaxed);
        READY.store(true, Ordering::Release);
    });
    while !READY.load(Ordering::Acquire) {
        thread::sleep(std::time::Duration::from_millis(100));
        println!("Not ready yet...");
    }
    println!("DATA: {}", DATA.load(Ordering::Relaxed));
}
