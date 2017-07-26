// https://rustbyexample.com/flow_control/match/destructuring/destructure_structures.html
// http://rust-lang-ja.org/rust-by-example/flow_control/match/destructuring/destructure_structures.html

fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    // destructure members of the struct
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {}, y = {}", a, b, y);

    // you can destructure structs and rename the variables,
    // the order is not important

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // and you can also ignore some variables:
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // this will give an error: pattern doesn not mention field `x`
    // let Foo { y } = foo;
}
