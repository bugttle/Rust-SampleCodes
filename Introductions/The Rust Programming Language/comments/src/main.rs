// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/comments.html
#![allow(unused_variables, dead_code)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Comments

    // Line comments are anything after '//' and extend to the end of line.

    let x = 5; // This is also a line comment.

    // If you have a long explanation for something, you can put line comments next
    // to each other. Put a space between the // and your comment so that it's
    // more readable.

    /// Adds one to the numbergiven.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(6, add_one(5));
    /// # fn add_one(x: i32) -> i32 {
    /// #     x + 1
    /// # }
    /// ```
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // `//!` style comments can't use here.
    // //! # The Rust Standard Library
    // //!
    // //! The Rust Standard Library provides the essential runtime
    // //! functionality for building portable Rust software.
}
