// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/testing.html
#![allow(unused_imports)]

//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    ////////////////////////////////////////////////////////////////////////////////
    // Testing

    ////////////////////////////////////////////////////////////////////////////////
    // The `test` attribute

    #[test]
    fn it_works1() {
    }

    #[test]
    fn it_works2() {
        // Linux:   $ echo $?
        // Windows: echo %ERRORLEVEL%

        // it should be failed: thread 'tests::it_works2' panicked at 'assertion failed: false', src/lib.rs:41
        // assert!(false);
    }

    #[test]
    #[should_panic]
    fn it_workd3() {
        assert!(false);
    }

    #[test]
    #[should_panic]
    fn it_workd4() {
        assert_eq!("Hello", "world");
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn it_workd5() {
        assert_eq!("Hello", "world");
    }

    pub fn add_two2(a: i32) -> i32 {
        a + 2
    }

    #[test]
    fn it_works6() {
        assert_eq!(4, add_two2(2));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // The `ignore` attribute

    #[test]
    fn it_works7() {
        assert_eq!(4, add_two2(2));
    }
    
    #[test]
    #[ignore]
    fn expensive_test() {
        // Code that takes an hour to run...
        // The expensive tests can be run explicitly using `cargo test -- --ignored`:
    }

    ////////////////////////////////////////////////////////////////////////////////
    // The `tests` module

    use super::*;

    #[test]
    fn it_works8() {
        assert_eq!(4, add_two(2));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // The `tests` module

    use super::*;

    #[test]
    fn it_works9() {
        assert_eq!(4, add_two(2));
    }
}
