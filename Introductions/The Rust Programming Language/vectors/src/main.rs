// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/vectors.html
#![allow(unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Vectors

    let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
    let v = vec![0; 10]; // A vector of ten zeroes.

    ////////////////////////////////////////////////////////////////////////////////
    // Accessing elements

    println!("The third element of v is {}", v[2]);

    let i: usize = 0;
    let j: i32 = 0;

    // Works:
    v[i];

    // Doesn't: error: the trait bound `std::vec::Vec<{integer}>: std::ops::Index<i32>` is not satisfied
    // v[j];

    ////////////////////////////////////////////////////////////////////////////////
    // Iterating

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}
