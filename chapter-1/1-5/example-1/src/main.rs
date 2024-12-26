use std::cell::Cell;

fn main() {
    let a = Cell::new(42);
    // let b = Cell::new(42);
    // f(&a, &b);
    f(&a, &a);

    let v = Cell::new(vec![1, 2, 3]);
    y(&v);
    for n in v.take() {
        println!("{}", n);
    }
}

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();

    if before != after {
        x();
    }
}

fn x() {
    println!("Hello from x function!");
}

fn y(v: &Cell<Vec<i32>>) {
    let  mut v2 = v.take();
    v2.push(42);
    v.set(v2);
}
