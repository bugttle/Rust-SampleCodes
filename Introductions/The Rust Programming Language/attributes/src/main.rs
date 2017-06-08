// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/attributes.html
#![allow(dead_code)]

////////////////////////////////////////////////////////////////////////////////
// Attributes

// #[foo]
// struct Foo;
//
// mod bar {
//     #![bar]
// }

#[test]
fn check() {
    assert_eq!(2, 1 + 1);
}

#[inline(always)]
fn super_fast_fn() {
}

#[cfg(target_os = "macos")]
mod macos_only {
}

fn main() {
}
