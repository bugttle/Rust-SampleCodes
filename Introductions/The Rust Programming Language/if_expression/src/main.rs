// https://doc.rust-lang.org/book/if.html
#![allow(unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // if

    let x = 5;

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    ////////////////////////////////////////////////////////////////////////////////

    let x = 5;

    let y = if x == 5 {
        10
    } else {
        15
    }; // y: i32

    ////////////////////////////////////////////////////////////////////////////////

    let x = 5;

    let y = if x == 5 { 10 } else { 15 }; // y: i32
}
