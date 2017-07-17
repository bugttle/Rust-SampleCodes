// https://rustbyexample.com/custom_types/constants.html
// http://rust-lang-ja.org/rust-by-example/custom_types/constants.html

// Globals are declared outside all other scopes.
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n){ "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5; // error[E0070]: invalid left-hand side expression
    // FIXME ^ Comment out this line
}
