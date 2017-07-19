// https://rustbyexample.com/variable_bindings/mut.html
// http://rust-lang-ja.org/rust-by-example/variable_bindings/mut.html

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1; // error[E0384]: re-assignment of immutable variable `_immutable_binding`
    // FIXME ^ Comment out this line
}
