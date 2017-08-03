// https://rustbyexample.com/crates/lib.html
// http://rust-lang-ja.org/rust-by-example/crates/lib.html

// $ rustc --crate-type=lib rary.rs

// rary.rs
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that'n> ");

    private_function();
}
