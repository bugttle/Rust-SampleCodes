// https://rustbyexample.com/scope/move/mut.html
// http://rust-lang-ja.org/rust-by-example/scope/move/mut.html

fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;
    // error: cannot assign to immutable `Box` content `*immutable_box`

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contains of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
