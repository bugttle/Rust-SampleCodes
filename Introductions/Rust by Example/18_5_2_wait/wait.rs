// https://rustbyexample.com/std_misc/process/wait.html
// http://rust-lang-ja.org/rust-by-example/std_misc/process/wait.html

use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
