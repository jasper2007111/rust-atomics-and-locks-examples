use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;
use std::thread;
use std::time::Duration;

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        // A background thread to process all 100 items
        s.spawn(|| {
            for i in 0..100 {
                thread::sleep(Duration::from_secs(1));
                num_done.store(i + 1, Ordering::Relaxed);
                main_thread.unpark(); // Wake up the main thread
            }
        });

        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {}/100 done", n);
            thread::park_timeout(Duration::from_secs(1)); // 可中断的休眠
        }
    });
    println!("Done!");
}
