use std::thread;

fn main() {
    for i in 0..2000 {
        println!("i:{}", i);
        thread::spawn(move|| {
            let id = allocate_new_id();
            println!("New ID: {}, i:{i}", id);
        }).join().unwrap();
    }

    println!("Done!");
}

fn allocate_new_id() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    assert!(id < 1000, "too many ids."); // 这个不是线程安全的，所以有可能会出现书中作者所说的问题
    id
}
