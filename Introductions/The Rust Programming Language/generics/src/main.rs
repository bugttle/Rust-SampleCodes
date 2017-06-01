// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/generics.html
#![allow(dead_code, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Generics

    let x: Option<i32> = Some(5);
    // let x: Option<f64> = Some(5); // error: mismatched types
    let x: Option<f64> = Some(5.0f64);

    ////////////////////////////////////////////////////////////////////////////////
    // Generics functions

    fn takes_anything<T>(x: T) {
        // Do something with `x`.
    }

    fn takes_two_things<T, U>(x: T, y: U) {
        // ...
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Generics structs

    struct Point<T> {
        x: T,
        y: T,
    }

    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };

    impl<T> Point<T> {
        fn swap(&mut self) {
            std::mem::swap(&mut self.x, &mut self.y);
        }
    }
}
