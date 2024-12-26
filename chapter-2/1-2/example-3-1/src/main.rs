fn main() {
    let mut id: u32;
    loop {
        id = allocate_new_id();
        if id == 0 {
            println!("No more IDs available!");
            break;
        }
        println!("New ID: {}", id);
    }
    println!("Done!");
}

fn allocate_new_id() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static NEXT_ID: AtomicU32 = AtomicU32::new(u32::MAX-100); // 为了方便测试，这里设置了一个较大的值
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}