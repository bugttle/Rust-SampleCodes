// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/loops.html
#![allow(unreachable_code)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // loop

    loop {
        println!("Loop forever!");
    }

    ////////////////////////////////////////////////////////////////////////////////
    // while

    let mut x = 5; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        x += x -3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // for

    for x in 0..10 {
        println!("{}", x); // x: i32
    }

    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Ending iteration early

    let mut x = 5;

    loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 { break; }
    }

    for x in 0..10 {
        if x % 2 == 0 { continue; }

        println!("{}", x);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Loop labels

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
            if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }
}
