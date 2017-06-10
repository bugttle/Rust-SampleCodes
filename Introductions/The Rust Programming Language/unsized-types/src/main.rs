// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/unsized-types.html
#![allow(dead_code)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Unsized Types

    // impl Foo for str {
    // }

    // impl<T> Foo for [T] {
    // }

    // impl Foo for &str {
    // }

    ////////////////////////////////////////////////////////////////////////////////
    // ?Sized

    struct Foo<T: ?Sized> {
        f: T,
    }
}
