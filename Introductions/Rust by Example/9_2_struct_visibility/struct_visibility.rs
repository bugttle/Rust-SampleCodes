// https://rustbyexample.com/mod/struct_visibility.html
// http://rust-lang-ja.org/rust-by-example/mod/struct_visibility.html

mod my {
    // A public struct with a public field of generic type `T`
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    let white_box = my::WhiteBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The white box contains: {}", white_box.contents);

    // Public structs with private fields cannot be constructed using field name.
    // Error! `BlackBox` has private fields
    //let black_box = my::BlackBox { contents: "classified informaion" };
    // TODO ^ Try uncommenting this line
    // error[E0451]: field `contents` of struct `my::BlackBox` is private

    // However, structs with private fields can be created using
    // public constructors
    let _black_box = my::BlackBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The black box contains: {}", _black_box.contents);
    // TODO ^ Try uncommenting this line
    // error: field `contents` of struct `my::BlackBox` is private
}
