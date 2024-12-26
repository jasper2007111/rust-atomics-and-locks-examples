fn main() {
    let a = 42;
    let mut b = 42;

    f(&a, &mut b);
}

fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;

    if before != after {
        x(); // This will never be called
    }
}

fn x() {
    println!("Hello from x function!");
}
