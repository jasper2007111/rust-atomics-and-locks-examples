use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

fn main() {
    let x = get_x();
    thread::spawn(move || {
        let x = get_x();
        println!("x1 = {}", x);
    })
    .join()
    .unwrap();

    thread::spawn(move || {
        let x = get_x();
        println!("x2 = {}", x);
    })
    .join()
    .unwrap();

    println!("Done！x = {}", x);
}

fn get_x() -> u64 {
    use std::sync::Once;
    
    static START: Once = Once::new();
    let mut x: u64 = 0;
    START.call_once(|| {
        x = calculate_x();
    });

    x
}

fn calculate_x() -> u64 {
    println!("Computing...");
    thread::sleep(Duration::from_secs(2));
    42
}