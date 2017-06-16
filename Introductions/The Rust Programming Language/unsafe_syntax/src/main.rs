// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/unsafe.html
#![allow(dead_code, unused_unsafe)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Unsafe

    unsafe fn danger_will_robinson() {
        // Scary stuff...
    }

    unsafe {
        // Scary stuff...
    }

    unsafe trait Scary { }

    unsafe impl Scary for i32 {}
}
