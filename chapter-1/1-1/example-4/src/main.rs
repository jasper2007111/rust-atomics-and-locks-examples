use std::thread;

fn main() {
    let numbers = Vec::from_iter(1..1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        // let sum = numbers.into_iter().fold(0, |acc, n| acc + n);
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("Average: {}", average);
}
