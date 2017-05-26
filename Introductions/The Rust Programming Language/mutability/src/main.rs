// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/mutability.html
#![allow(dead_code, unused_assignments, unused_mut, unused_variables)]


fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Mutability

    let x = 5;
    // x = 6; // error: re-assignment of immutable variable `x`

    let mut x = 5;
    x = 6; // No problem!

    let mut x = 5;
    let y = &mut x;

    let mut x = 5;
    let mut y = &mut x;

    let (mut x, y) = (5, 6);
    fn foo(mut x: i32) {
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Interior vs. Exterior Mutability

    use std::sync::Arc;
    let x = Arc::new(5);
    let y = x.clone();

    use std::cell::RefCell;
    let x = RefCell::new(42);
    let y = x.borrow_mut();
    // let z = x.borrow_mut(); // thread 'main' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:859

    ////////////////////////////////////////////////////////////////////////////////
    // Field-level mutability

    struct Point1 {
        x :i32,
        // mut y: i32, // Nope. error: expected identifier, found keyword `mut`
    }

    struct Point2 {
        x: i32,
        y: i32,
    }
    let mut a = Point2 { x: 5, y: 6 };
    a.x = 10;
    let b = Point2 { x: 5, y: 6 };
    // b.x = 10; // Error: cannot assign to immutable field `b.x`.

    use std::cell::Cell;
    struct Point3 {
        x: i32,
        y: Cell<i32>,
    }
    let point = Point3 { x: 5, y: Cell::new(6) };
    point.y.set(7);
    println!("y: {:?}", point.y);
}
