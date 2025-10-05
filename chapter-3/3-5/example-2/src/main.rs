use std::thread;
static mut DATA: String = String::new();
static LOCKED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn f() {
    if LOCKED
        .compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::Acquire,
            std::sync::atomic::Ordering::Relaxed,
        )
        .is_ok()
    {
        unsafe {
            DATA.push_str("hello");
        }
        LOCKED.store(false, std::sync::atomic::Ordering::Release);
    } else {
        println!("Data is locked, cannot modify");
    }
}
fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
}
