// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/borrow-and-asref.html
#![allow(dead_code, unused_variables)]

////////////////////////////////////////////////////////////////////////////////
// Borrow and AsRef

use std::collections::HashMap;
use std::borrow::Borrow;
use std::fmt::Display;

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Borrow

    let mut map = HashMap::new();
    map.insert("Foo".to_string(), 42);

    assert_eq!(map.get("Foo"), Some(&42));


    fn foo<T: Borrow<i32> + Display>(a: T) {
        println!("a is borrowed: {}", a);
    }

    let mut i = 5;

    foo(&i);
    foo(&mut i);


    ////////////////////////////////////////////////////////////////////////////////
    // AsRef

    let s = "Hello".to_string();

    fn foo_as_ref<T: AsRef<str>>(s: T) {
        let slice = s.as_ref();
    }
}
