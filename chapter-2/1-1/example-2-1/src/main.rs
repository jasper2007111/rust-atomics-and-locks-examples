use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        // A background thread to prosecc all 100 items
        s.spawn(|| {
            for i in 0..100 {
                thread::sleep(Duration::from_secs(1));
                num_done.store(i + 1, Ordering::Relaxed);
            }
        });

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
