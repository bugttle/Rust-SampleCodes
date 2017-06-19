// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/conditional-compilation.html

////////////////////////////////////////////////////////////////////////////////
// Conditional Compilation

// #[cfg(foo)]
// #[cfg(bar = "baz")]

// #[cfg(any(unix, windows))]
// #[cfg(all(unix, target_pointer_width = "32"))]
// #[cfg(not(foot))]

// #[cfg(any(not(unix), all(target_os="macos", target_arch = "powerpc")))] 

#[cfg(feature = "foo")]
mod foo {
}

////////////////////////////////////////////////////////////////////////////////
// cfg_attr

#[cfg_attr(a, b)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // cfg!

    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        println!("Think Different!");
    }
}
