// https://doc.rust-lang.org/book/references-and-borrowing.html
#![allow(unused_variables, unused_mut)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Borrowing

    fn foo1(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // Do stuff with `v1` and `v2`.
        // Return the answer.
        42
    }
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let answer = foo1(&v1, &v2);

    // We can use `v1` and `v2` here!


    fn foo2(v: &Vec<i32>) {
        // v.push(5); // error: cannot borrow immutable borrowed content `*v` as mutable
    }
    let v = vec![];
    foo2(&v);

    ////////////////////////////////////////////////////////////////////////////////
    // &mut references

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

    ////////////////////////////////////////////////////////////////////////////////
    // Thinking in scopes

    let mut x = 5;
    let y = &mut x;
    *y += 1;
    // println!("{}", x); // error: cannot borrow `x` as immutable because it is also borrowed as mutable

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);

    ////////////////////////////////////////////////////////////////////////////////
    // Iterator invalidation

    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
        // v.push(34); // error: cannot borrow `v` as mutable because it is also borrowed as immutable
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Use after free

    let y: &i32;
    {
        let x = 5;
        // y = &x; // error: `x` does not live long enough
    }

    let y: &i32;
    let x = 5;
    // y = &x; // error: `x` does not live long enough
}
