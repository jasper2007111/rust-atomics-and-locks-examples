use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        for n in numbers {
            println!("{n}"); // n是不是应该高亮？
        }
    }).join().unwrap();
}
