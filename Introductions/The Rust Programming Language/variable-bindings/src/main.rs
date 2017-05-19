// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/variable-bindings.html
#![allow(unused_variables, unused_assignments)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////<
    // Patterns
    let (x, y) = (1, 2);

    ////////////////////////////////////////////////////////////////////////////////<
    // Type annotations
    let x: i32 = 5;
    let x = 5;

    ////////////////////////////////////////////////////////////////////////////////<
    // Mutability
    let mut x = 5;
    x = 10;

    ////////////////////////////////////////////////////////////////////////////////<
    // Initializing bindings
    let x: i32;
    // println!("The value of x is: {}", x); => You'll get an error.

    ////////////////////////////////////////////////////////////////////////////////<
    // Scope and shadowing
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // println!("The value of x is {} and value of y is {}", x, y); => This won't work.
    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8".
        let x = 12;
        println!("{}", x); // Prints "12".
    }
    println!("{}", x); // Prints "8".
    let x = 42;
    println!("{}", x); // Prints "42".

    let mut x: i32 = 1;
    x = 7;
    let x = x; // `x` is now immutable and is bound to `7`.

    let y = 4;
    let y = "I can also be bound to text!"; // `y` is now of a different type.
}
