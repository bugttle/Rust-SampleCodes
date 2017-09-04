// https://rustbyexample.com/flow_control/loop/return.html
// http://rust-lang-ja.org/rust-by-example/std/hash.html

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // rustc 1.17.0 (56124baa9 2017-04-24)
            // error: `break` with a value is experimental (see issue #37339)
        }
    };

    assert_eq!(result, 20);
}
