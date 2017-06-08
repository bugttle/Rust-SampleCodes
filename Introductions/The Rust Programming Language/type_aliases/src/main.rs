// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/type-aliases.html
#![allow(dead_code, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Type Aliases

    type Name = String;
    let x: Name = "Hello".to_string();

    let x: i32 = 5;
    let y: i64 = 5;

    // if x == y { // error[E0308]: mismatched types
    //     // ...
    // }


    type Num = i32;

    let x: i32 = 5;
    let y: Num = 5;

    if x == y {
        // ...
    }

    use std::result;

    enum ConcreteError {
        Foo,
        Bar,
    }

    type Result<T> = result::Result<T, ConcreteError>;
}
