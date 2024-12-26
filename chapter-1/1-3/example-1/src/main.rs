use std::thread;

fn main() {
    static X: [i32; 3] = [1, 2, 3];

    thread::spawn(|| {
        dbg!(&X); // dbg!(X);这样也行
    }).join().unwrap();

    thread::spawn(|| {
        dbg!(&X);
    }).join().unwrap();
}
