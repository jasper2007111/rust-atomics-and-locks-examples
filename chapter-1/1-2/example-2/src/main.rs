use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            numbers.push(4);
        });
        s.spawn(|| {
            numbers.push(5); // two closures require unique access to `numbers` at the same time
        });
    });
}
