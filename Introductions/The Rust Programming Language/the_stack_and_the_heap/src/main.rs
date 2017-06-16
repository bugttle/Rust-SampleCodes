// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/the-stack-and-the-heap.html
#![allow(unused_variables)]

////////////////////////////////////////////////////////////////////////////////
// The Stack and the Heap

fn foo(x: &i32) {
    let y = 10;
    let z = &y;
    
    baz(z);
    bar(x, z);
}

fn bar(a: &i32, b: &i32) {
    let c = 5;
    let d = Box::new(5);
    let e = &d;

    baz(e);
}

fn baz(f: &i32) {
    let g = 100;
}

fn main() {
    let h = 3;
    let i = Box::new(20);
    let j = &h;

    foo (j);
}
