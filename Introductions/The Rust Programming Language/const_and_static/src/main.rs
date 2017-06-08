// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/const-and-static.html
#![allow(dead_code)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // const and static

    const N_CONST: i32 = 5;

    ////////////////////////////////////////////////////////////////////////////////
    // static

    static N_STATIC: i32 = 5;
    static NAME: &'static str = "Steve";

    ////////////////////////////////////////////////////////////////////////////////
    // Mutability

    static mut N_MUT: i32 = 5;

    unsafe {
        N_MUT += 1;

        println!("N: {}", N_MUT);
    }
}
