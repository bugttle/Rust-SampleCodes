// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/strings.html
#![allow(unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Strings

    let greeting = "Hello there."; // greeting: &'static str


    let s = "foo
        bar";
    assert_eq!("foo\n        bar", s);

    let s = "foo\
        bar";
    assert_eq!("foobar", s);


    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);


    fn takes_slice(slice: &str) {
        println!("Got: {}", slice);
    }

    let s = "Hello".to_string();
    takes_slice(&s);


    // use std::net::TcpStream;
    // TcpStream::connect("192.168.0.1:3000"); // Parameter is of type &str.
    // let addr_string = "192.168.0.1:3000".to_string();
    // TcpStream::connect(&*addr_string); // Convert `addr_string` to &str.

    ////////////////////////////////////////////////////////////////////////////////
    // Indexing

    let s = "hello";
    // println!("The first letter of s is {}", s[0]); // ERROR!!! error: the trait bound `str: std::ops::Index<{integer}>` is not satisfied


    let hachiko = "忠犬ハチ公";
    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }
    println!("");

    for c in hachiko.chars() {
        print!("{}, ", c);
    }
    println!("");

    ////////////////////////////////////////////////////////////////////////////////
    // Slicing

    let dog = "hachiko";
    let hachi = &dog[0..5];

    let dog = "忠犬ハチ公";
    // let hachi = &dog[0..2]; // thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside '忠' (bytes 0..3) of `忠犬ハチ公`'

    ////////////////////////////////////////////////////////////////////////////////
    // Concatenation

    let hello = "Hello".to_string();
    let world = "world!";

    let hello_world = hello + world;

    let hello = "Hello".to_string();
    let world = "world!".to_string();

    let hello_world = hello + &world;
}
