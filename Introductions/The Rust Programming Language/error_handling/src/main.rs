// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/error-handling.html
#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

////////////////////////////////////////////////////////////////////////////////
// Error Handling

use std::env;
use std::num::ParseIntError;
use std::result;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::fmt;
use std::num;
use std::error;
use std::error::Error;
use std::fs;

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


    ////////////////////////////////////////////////////////////////////////////////
    // Working with multiple error types


    ////////////////////////////////////////////////////////////////////////////////
    // Composing `Option` and `Result`

    fn double_arg(mut argv: env::Args) -> Result<i32, String> {
        argv.nth(1)
            .ok_or("Please give at least one argument".to_owned())
            .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
            .map(|n| 2 * n)
    }

    match double_arg(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }


    ////////////////////////////////////////////////////////////////////////////////
    // The limits of combinators

    fn file_double<P: AsRef<Path>>(file_path: P) -> i32 {
        let mut file = File::open(file_path).unwrap(); // error 1
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap(); // error 2
        let n: i32 = contents.trim().parse().unwrap(); // error 3
        2 * n
    }

    // let doubled = file_double("foobar");
    // println!("{}", doubled);


    fn file_double2<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
        File::open(file_path)
            .map_err(|err| err.to_string())
            .and_then(|mut file| {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .map_err(|err| err.to_string())
                    .map(|_| contents)
            })
            .and_then(|contents| {
                contents.trim().parse::<i32>()
                        .map_err(|err| err.to_string())
            })
            .map(|n| 2 * n)
    }

    match file_double2("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Early returns

    fn file_double3<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => return Err(err.to_string()),
        };
        let mut contents = String::new();
        if let Err(err) = file.read_to_string(&mut contents) {
            return Err(err.to_string());
        }
        let n: i32 = match contents.trim().parse() {
            Ok(n) => n,
            Err(err) => return Err(err.to_string()),
        };
        Ok(2 * n)
    }

    match file_double3("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }


    ////////////////////////////////////////////////////////////////////////////////
    // The `try!` macro

    fn file_double4<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
        let mut file = try!(File::open(file_path).map_err(|e| e.to_string()));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
        let n = try!(contents.trim().parse::<i32>().map_err(|e| e.to_string()));
        Ok(2 * n)
    }

    match file_double4("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Defining your own error type

    // We derivce `Debug` because all types should probably derive `Debug`.
    // This gives us a reasonable human readable description of `CliError` values.
    #[derive(Debug)]
    enum CliError {
        Io(io::Error),
        Parse(num::ParseIntError),
    }

    fn file_double5<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
        let mut file = try!(File::open(file_path).map_err(CliError::Io));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents).map_err(CliError::Io));
        let n: i32 = try!(contents.trim().parse().map_err(CliError::Parse));
        Ok(2 * n)
    }

    match file_double5("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Standard library traits used for error handling


    ////////////////////////////////////////////////////////////////////////////////
    // The `Error` trait

    impl fmt::Display for CliError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                // Both underlyng errors already impl `Display`, so we defer to
                // heir implementations.
                CliError::Io(ref err) => write!(f, "IO error: {}", err),
                CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
            }
        }
    }

    impl error::Error for CliError {
        fn description(&self) -> &str {
            // Both underlying errors already impl `Error`, so we defer to their
            // implementations.
            match *self {
                CliError::Io(ref err) => err.description(),
                CliError::Parse(ref err) => err.description(),
            }
        }

        fn cause(&self) -> Option<&error::Error> {
            match *self {
                // N.B. Both of these implicitly cast `err` from their concrete
                // types (eigher `&io::Error` or `&num::ParseIntError`)
                // to a trait object `&Error`. This works because both error types
                // implement `Error`.
                CliError::Io(ref err) => Some(err),
                CliError::Parse(ref err) => Some(err),
            }
        }
    }


    ////////////////////////////////////////////////////////////////////////////////
    // The `From` trait

    let string: String = From::from("foo");
    let bytes: Vec<u8> = From::from("foo");
    let cow: ::std::borrow::Cow<str> = From::from("foo");

    // We have to jump through some hoops to actually get error values:
    let io_err: io::Error = io::Error::last_os_error();
    let parse_err: num::ParseIntError = "not a number".parse::<i32>().unwrap_err();

    // OK, here are the conversions:
    let err1: Box<Error> = From::from(io_err);
    let err2: Box<Error> = From::from(parse_err);

    fn file_double6<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
        let mut file = try!(File::open(file_path).map_err(|e| e.to_string()));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
        let n = try!(contents.trim().parse::<i32>().map_err(|e| e.to_string()));
        Ok(2 * n)
    }

    fn file_double7<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<Error>> {
        let mut file = try!(File::open(file_path));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        let n = try!(contents.trim().parse::<i32>());
        Ok(2 * n)
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Composing custom error types

    impl From<io::Error> for CliError {
        fn from(err: io::Error) -> CliError {
            CliError::Io(err)
        }
    }

    impl From<num::ParseIntError> for CliError {
        fn from(err: num::ParseIntError) -> CliError {
            CliError::Parse(err)
        }
    }

    fn file_double8<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
        let mut file = try!(File::open(file_path));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        let n : i32 = try!(contents.trim().parse());
        Ok(2 * n)
    }

    enum CliErrorWithFloat {
        Io(io::Error),
        ParseInt(num::ParseIntError),
        ParseFloat(num::ParseFloatError),
    }

    impl From<num::ParseFloatError> for CliErrorWithFloat {
        fn from(err: num::ParseFloatError) -> CliErrorWithFloat {
            CliErrorWithFloat::ParseFloat(err)
        }
    }
}
