use std::thread;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicU64;

fn main() {
    for i in 0..100 {
        println!("i:{}", i);
        thread::spawn(move|| {
            let id = get_key();
            println!("New ID: {}, i:{i}", id);
        }).join().unwrap();
    }

    println!("Done!");
}

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Ordering::Relaxed);
    if key == 0 {
        let new_key = rand::random();
        match KEY.compare_exchange_weak(0, new_key, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return new_key,
            Err(x) => return x,
        }
    } else {
        key
    }
}
