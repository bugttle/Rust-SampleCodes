// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/functions.html
#![allow(unused_variables, dead_code)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Functions
    print_number(5);
    print_sum(5, 6);

    fn add_one(x: i32) -> i32 {
        x + 1
    }
    add_one(5);

    ////////////////////////////////////////////////////////////////////////////////
    // Early returns
    // return;

    ////////////////////////////////////////////////////////////////////////////////
    // Diverging functions
    fn diverges() -> ! {
        panic!("This function never returns!");
    }
    // You can use "RUST_BACKTRACE=1"
    // diverges();
    // let x: i32 = diverges();
    // let x: String = diverges();

    ////////////////////////////////////////////////////////////////////////////////
    // Function pointers
    fn plus_one(i: i32) -> i32 {
        i + 1
    }
    let f: fn(i32) -> i32 = plus_one;
    let f = plus_one;
    let six = f(5);
}

fn print_number(x: i32) {
    println!("x is {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is {}", x + y);
}
