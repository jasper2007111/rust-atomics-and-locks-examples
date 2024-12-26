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
    let mut id = NEXT_ID.load(Ordering::Relaxed);
    loop {
        assert!(id < 1000, "too many ids.");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return id,
            Err(x) => id = x,
        }
    }
}
