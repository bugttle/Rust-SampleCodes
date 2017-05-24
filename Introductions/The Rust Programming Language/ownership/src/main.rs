// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/ownership.html
#![allow(dead_code, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Ownership

    fn foo() {
        let v = vec![1, 2, 3];
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Move semantics
    let v = vec![1, 2, 3];
    let v2 = v;
    // println!("v[0] is: {}", v[0]); // error: use of moved value: `v`

    fn take(v: Vec<i32>) {
        // What happens here isn't important.
    }
    let v = vec![1, 2, 3];
    take(v);

    // println!("v[0] is: {}", v[0]); // error: use of moved value: `v`

    ////////////////////////////////////////////////////////////////////////////////
    // Copy types

    let v = 1;
    let v2 = v;
    println!("v is: {}", v);

    let a = 5;
    let _y = double(a);
    println!("{}", a);

    fn double(x: i32) -> i32 {
        x * 2
    }

    let a = true;
    let _y = change_truth(a);
    println!("{}", a);

    fn change_truth(x: bool) -> bool {
        !x
    }
}
