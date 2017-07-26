// https://rustbyexample.com/flow_control/for.html
// http://rust-lang-ja.org/rust-by-example/flow_control/for.html

fn main() {
    // `n` will take the vaues: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
