use std::sync::atomic::fence;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;

static mut DATA: [u64; 10] = [0; 10];
const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn some_calculation(i: usize) -> u64 {
    (i as u64 + 1) * 10
}

fn main() {
    for i in 0..10 {
        std::thread::spawn(move || {
            let data = some_calculation(i);
            unsafe {
                DATA[i] = data;
            }
            READY[i].store(true, Ordering::Release);
        });
    }

    thread::sleep(std::time::Duration::from_millis(500));
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Ordering::Relaxed));
    if ready.contains(&true) {
        fence(Ordering::Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("DATA[{}]: {}", i, unsafe { DATA[i] });
            }
        }
    }
}
