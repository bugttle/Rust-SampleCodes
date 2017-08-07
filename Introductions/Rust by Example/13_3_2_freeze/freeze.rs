// https://rustbyexample.com/scope/borrow/freeze.html
// http://rust-lang-ja.org/rust-by-example/scope/borrow/freeze.html

fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Borrow `_mutable_integer`
        let _large_integer = &_mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        //_mutable_integer = 50;
        // FIXME ^ Comment out this line
        // error[E0506]: cannot assign to `_mutable_integer` because it is borrowed

        // `_large_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
