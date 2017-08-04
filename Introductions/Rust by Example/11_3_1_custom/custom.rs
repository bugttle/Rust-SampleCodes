// https://rustbyexample.com/attribute/cfg/custom.html
// http://rust-lang-ja.org/rust-by-example/attribute/cfg/custom.html

// $ rustc custom.rs
// => error[E0425]: cannot find function `condition_function` in this scope

// $ rustc --cfg some_condition custom.rs && ./custom

// custom.rs
#[cfg(some_condition)]
fn condition_function() {
    println!("condition met!")
}

fn main() {
    condition_function();
}
