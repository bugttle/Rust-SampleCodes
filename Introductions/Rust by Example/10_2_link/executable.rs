// https://rustbyexample.com/crates/link.html
// http://rust-lang-ja.org/rust-by-example/crates/link.html

// $ cd ../10_1_lib/
// $ rustc --crate-type=lib rary.rs
// $ cd ../10_2_link/
// $ rustc --extern rary=../10_1_lib/library.rlib executable.rs

// executable.rs
// Link to `library`, import items under the `rary` module
extern crate rary;

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}
