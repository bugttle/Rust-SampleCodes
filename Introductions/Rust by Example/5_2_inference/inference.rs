// https://rustbyexample.com/cast/inference.html
// http://rust-lang-ja.org/rust-by-example/cast/inference.html

fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem); // it will get error if this line commented out: error[E0282]: type annotations needed
    // Aha! Now the compiler knows that `veec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}
