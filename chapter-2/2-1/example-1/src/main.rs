use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = thread::spawn(|| while !STOP.load(Ordering::Relaxed) {
        some_work();
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            _ => println!("Unknown command"),
        }
    }

    STOP.store(true, Ordering::Relaxed);
    background_thread.join().unwrap();
}

fn some_work() {
    // println!("Some work is done");
    thread::sleep(Duration::from_secs(1));
}
