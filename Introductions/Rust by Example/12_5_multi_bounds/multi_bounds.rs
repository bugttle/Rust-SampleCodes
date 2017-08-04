// https://rustbyexample.com/generics/multi_bounds.html
// http://rust-lang-ja.org/rust-by-example/generics/multi_bounds.html

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}", t);
    println!("u: `{:?}", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // TODO ^ Try uncommenting this.
    // error[E0277]: the trait bound `[{integer}; 3]: std::fmt::Display` is not satisfied

    compare_types(&array, &vec);
}
