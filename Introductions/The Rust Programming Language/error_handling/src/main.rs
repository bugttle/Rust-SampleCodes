// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/error-handling.html
#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

////////////////////////////////////////////////////////////////////////////////
// Error Handling

use std::env;
use std::num::ParseIntError;
use std::result;

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // The Basics

    // Guess a number between 1 and 10.
    // If it matches the number we had in mind, return `true`. Else, return `false`.
    fn guess(n: i32) -> bool {
        if n < 1 || n > 10 {
            // panic!("Invalid number: {}", n); // will panic
        }
        n == 5
    }

    guess(11);


    ////////////////////////////////////////////////////////////////////////////////
    // Unwrapping explained

    let mut argv = env::args();
    // will panic
    // let arg: String = argv.nth(1).unwrap(); // error 1 // will panic
    // let n: i32 = arg.parse().unwrap(); // error 2 // will panic
    // println!("{}", 2 * n);


    ////////////////////////////////////////////////////////////////////////////////
    // The `Option` type

    // Searches `haystack` for the Unicode character `needle`. If one is found, the
    // byte offsetof the character is returned. Otherwise, `None` is returned.
    fn find(haystack: &str, needle: char) -> Option<usize> {
        for (offset, c) in haystack.char_indices() {
            if c == needle {
                return Some(offset);
            }
        }
        None
    }

    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i+1..]),
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Composing `Option<T>` values

    // Returns the extension of the given file name, where the extension is defined
    // as all characters following the first `.`.
    // If `file_name` has no `.`, then `None` is returned.
    fn extension_explicit(file_name: &str) -> Option<&str> {
        match find(file_name, '.') {
            None => None,
            Some(i) => Some(&file_name[i+1..]),
        }
    }


    // Returns the extension of the given file name, where the extension is defined
    // as all characters following the first `.`.
    // If `file_name` has no `.`, then `None` is returned.
    fn extension(file_name: &str) -> Option<&str> {
        find(file_name, '.').map(|i| &file_name[i+1..])
    }

    fn unwrap_or<T>(option: Option<T>, default: T) -> T {
        match option {
            None => default,
            Some(value) => value,
        }
    }

    assert_eq!(extension("foobar.csv").unwrap_or("rs"), "csv");
    assert_eq!(extension("foobar").unwrap_or("rs"), "rs");


    fn file_path_ext_explicit(file_path: &str) -> Option<&str> {
        match file_name_func(file_path) {
            None => None,
            Some(name) => match extension(name) {
                None => None,
                Some(ext) => Some(ext),
            }
        }
    }

    fn file_name_func(file_path: &str) -> Option<&str> {
        // Implementation elided.
        unimplemented!()
    }

    fn file_path_ext(file_path: &str) -> Option<&str> {
        file_name_func(file_path).and_then(extension)
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Parsing integers

    fn double_number(number_str: &str) -> i32 {
        2 * number_str.parse::<i32>().unwrap()
    }

    // let n: i32 = double_number("10");
    // assert_eq!(n, 20);


    fn double_number2(number_str: &str) -> Result<i32, ParseIntError> {
        match number_str.parse::<i32>() {
            Ok(n) => Ok(2 * n),
            Err(err) => Err(err),
        }
    }

    fn double_number3(number_str: &str) -> Result<i32, ParseIntError> {
        number_str.parse::<i32>().map(|n| 2 * n)
    }

    match double_number3("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(err) => println!("Error: {:?}", err),
    }


    ////////////////////////////////////////////////////////////////////////////////
    // The `Result` type alias idiom

    // type Result<T> = result::Result<T, ParseIntError>;

    // fn double_number(number_str: &str) -> Result<i32> {
    //     unimplemented!();
    // }
}
