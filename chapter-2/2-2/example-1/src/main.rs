use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;
use std::thread;
use std::time::Duration;

fn main() {
    let num_done = &AtomicUsize::new(0);
    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    println!("Processing item {}", t*25 + i);
                    thread::sleep(Duration::from_secs(1));
                    num_done.fetch_add(1, Ordering::Relaxed);
                }
            });
        }

        // The main thread shows status updates, every second.
        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {}/100 done", n);
            thread::sleep(Duration::from_secs(1)); // 不可中断的休眠
        }
    });
    println!("Done!");
}
