// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/choosing-your-guarantees.html
#![allow(unused_variables)]

////////////////////////////////////////////////////////////////////////////////
// Choosing your Guarantees

use std::cell::Cell;
use std::cell::RefCell;

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Basic pointer types

    let x = Box::new(1);
    let y = x;
    // `x` is no longer accessible here.


    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());

    // let mut x = 1;
    // let y = &mut x;
    // let z = &mut x;
    // x = 2;
    // *y = 3;
    // *z = 4;
    // println!("{}", x);


    ////////////////////////////////////////////////////////////////////////////////
    // Cell types

    let x = RefCell::new(vec![1, 2, 3, 4]);
    {
        println!("{:?}", *x.borrow())
    }

    {
        let mut my_ref = x.borrow_mut();
        my_ref.push(1);
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Synchronous types

    // {
    //     let guard = mutex.lock();
    //     // `guard` dereferences mutably to the inner type.
    //     *guard += 1;
    // } // Lock is released when destructor runs.
}
