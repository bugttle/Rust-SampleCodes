// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/documentation.html
#![allow(dead_code)]

////////////////////////////////////////////////////////////////////////////////
// Documentation


////////////////////////////////////////////////////////////////////////////////
// Documenting source code

/// Constructs a new `Rc<T>`.
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
/// let five = Rc::new(5);
/// ```
// pub fn new(value: T) -> Rc<T> {
//     // Implementation goes here.
// }

/// The `Option` type. See [the module level documentation](index.html) for more.
enum Option<T> {
    /// No value
    None,
    /// Some value `T`
    Some(T),
}

// error[E0585]: found a documentation comment that doesn't document anything
// /// The `Option` type. See [the module level documentation](index.html) for more.
// enum Option<T> {
//     None, /// No value
//     Some(T), /// Some value `T`
// }


////////////////////////////////////////////////////////////////////////////////
// Writing documentation comments

/// Constructs a new `Rc<T>`.

///
/// Other details about constructing `Rc<T>`s, maybe describing complicated
/// semantics, maybe additional options, all kinds of stuff.
///


////////////////////////////////////////////////////////////////////////////////
// Writing documentation comments

/// # Panics

/// # Errors

/// # Safety

/// # Examples
///
/// ```
/// use std::rc::Rc;
/// 
/// let five = Rc::new(5);
/// ```

/// # Examples
///
/// Simple `&str` patterns:
///
/// ```
/// let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
/// assert_eq!(v, vec!["Mary", "had", "a", "little", "lamb"]);
/// ```
///
/// More complex patterns with a lambda:
///
/// ```
/// let v: Vec<&str> = "abc1def2ghi".split(|c: char| c.is_numeric()).collect();
/// assert_eq!(v, vec!["abc", "def", "ghi"]);
/// ```


////////////////////////////////////////////////////////////////////////////////
// Code block annotations

/// ```
/// println!("Hello, world");
/// ```

// ```c
// printf("Hello, world\n");
// ```


////////////////////////////////////////////////////////////////////////////////
// Documentation as tests

/// ```
/// println!("Hello, world");
/// ```

/// ```
/// use std::rc::Rc;
///
/// let five = Rc::new(5);
/// ```


////////////////////////////////////////////////////////////////////////////////
// Documenting macros

/// Panic with a given message unless an expression evaluates to true.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate foo;
/// # fn main() {
/// panic_unless!(1 + 1 == 2, “Math is broken.”);
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate foo;
/// # fn main() {
/// panic_unless!(true == false, “I’m broken.”);
/// # }
/// ```
#[macro_export]
macro_rules! panic_unless {
    ($condition:expr, $($rest:expr),+) => ({ if ! $condition { panic!($($rest),+); } });
}

/// use std::io;
/// let mut input = String::new();
/// try!(io::stdin().read_line(&mut input));

/// A doc test using try!
///
/// ```
/// use std::io;
/// # fn foo() -> io::Result<()> {
/// let mut input = String::new();
/// try!(io::stdin().read_line(&mut input));
/// # Ok(())
/// # }
/// ```


////////////////////////////////////////////////////////////////////////////////
// Running documentation tests

// $ rustdoc --test path/to/my/crate/root.rs
// # or
// $ cargo test


/// ```ignore
/// fn foo() {
/// ```

/// ```should_panic
/// assert!(false);
/// ```

/// ```no_run
/// loop {
///     println!("Hello, world");
/// }
/// ```


////////////////////////////////////////////////////////////////////////////////
// Documenting modules

mod foo {
    //! This is documentation for the `foo` module.
    //!
    //! # Examples

    // ...
}


////////////////////////////////////////////////////////////////////////////////
// `doc` attributes

// /// this
// #[doc="this"]
// 
// ///! this
// #![doc="this"]


////////////////////////////////////////////////////////////////////////////////
// Missing documentation

// #![warn(missing_docs)]
//
// #![deny(missing_docs)]
//
// #[allow(missing_docs)]
// struct Undocumented;
//
// #[doc(hidden)]
// struct Hidden;


////////////////////////////////////////////////////////////////////////////////
// Controlling HTML

// #![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
//        html_favicon_url = "https://www.rust-lang.org/favicon.ico",
//        html_root_url = "https://doc.rust-lang.org/")]


////////////////////////////////////////////////////////////////////////////////
// Configuring documentation tests

// #![doc(test(attr(allow(unused_variables), deny(warnings))))]


////////////////////////////////////////////////////////////////////////////////
// Generating options

// --html-in-header FILE:
//      includes the contents of FILE at the end of the <head>...</head> section.
// --html-before-content FILE:
//      includes the contents of FILE directly after <body>, before the rendered content (including the search bar).
// --html-after-content FILE:
//      includes the contents of FILE after all the rendered content.


////////////////////////////////////////////////////////////////////////////////
// Security note

// The Markdown in documentation comments is placed without processing into the final webpage. Be careful with literal HTML:
// /// <script>alert(document.cookie)</script>

fn main() {
}
