use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                let mut max = 0;
                for i in 0..25 {
                    let start = Instant::now();
                    println!("Processing item {}", t * 25 + i);
                    thread::sleep(Duration::from_secs(1));
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Ordering::Relaxed);
                    total_time.fetch_add(time_taken, Ordering::Relaxed);
                    max_time.fetch_max(time_taken, Ordering::Relaxed);
                }
                let current_max = max_time.load(Ordering::Relaxed);
                if max > current_max {
                    max_time.store(max, Ordering::Relaxed);
                }
            });
        }

        // The main thread shows status updates, every second.
        loop {
            let total_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            if n == 0 {
                println!("Working.. nothing done yet");
            } else {
                println!(
                    "Working.. {}/100 done, {:?} average, {:?} peak",
                    n,
                    total_time / n as u32,
                    max_time
                );
            }
            println!("Working.. {}/100 done", n);
            thread::sleep(Duration::from_secs(1)); // 不可中断的休眠
        }
    });
    println!("Done!");
}
