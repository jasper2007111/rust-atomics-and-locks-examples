use std::cell::RefCell;

fn main() {
    let v = RefCell::new(vec![1, 2, 3]);
    y(&v);
    for n in v.take() {
        println!("{}", n);
    }
}

fn y(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(42);
}
