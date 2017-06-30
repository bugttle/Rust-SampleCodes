// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/using-rust-without-the-standard-library.html
#![allow(dead_code)]

////////////////////////////////////////////////////////////////////////////////
// Using Rust Without the Standard Library

#![no_std]

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn may_fail(failure: bool) -> Result<(), &'static str> {
    if failure {
        Err("this didn't work!")
    } else {
        Ok(())
    }
}
