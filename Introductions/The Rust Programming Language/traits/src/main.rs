// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/traits.html
#![allow(dead_code)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Traits

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Trait bounds on generic functions

    // error: no method named `area` found for type `T` in the current scope
    // fr print_area<T>(shape: T) {
    //     println!("This shape has an area of {}", shape.area());
    // }

    fn print_area<T: HasArea>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }


    struct Square {
        x: f64,
        y: f64,
        side: f64,
    }

    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area(c);
    print_area(s);

    // print_area(5); // error: the trait bound `{integer}: main::HasArea` is not satisfied

    ////////////////////////////////////////////////////////////////////////////////
    // Trait bounds on generic functions

    struct Rectangle<T> {
        x: T,
        y: T,
        width: T,
        height: T,
    }

    impl<T: PartialEq> Rectangle<T> {
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }

    let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };

    assert!(r.is_square());

    r.height = 42;
    assert!(!r.is_square());

    ////////////////////////////////////////////////////////////////////////////////
    // Rules for implementating traits

    impl HasArea for i32 {
        fn area(&self) -> f64 {
            println!("this is silly");

            *self as f64
        }
    }

    5.area();


    // let mut f = std::fs::File::open("foo.txt").expect("Couldn't open foo.txt");
    // let buf = b"whatever"; // buf: &[u8; 8], a byte string literal.
    // // let result = f.write(buf); // error: no method named `write` found for type `std::fs::File` in the current scope

    // use std::io::Write;
    // let mut f = std::fs::File::open("foo.txt").expect("Couldn't open foo.txt");
    // let buf = b"whatever";
    // let result = f.write(buf);

    ////////////////////////////////////////////////////////////////////////////////
    // Multiple trait bounds

    fn foo1<T: Clone>(x: T) {
        x.clone();
    }

    use std::fmt::Debug;

    fn foo2<T: Clone + Debug>(x: T) {
        x.clone();
        println!("{:?}", x)
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Where clause

    fn foo3<T: Clone, K: Clone + Debug>(x: T, y: K) {
        x.clone();
        y.clone();
        println!("{:?}", y);
    }

    fn bar1<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
        x.clone();
        y.clone();
        println!("{:?}", y);
    }

    foo3("Hello", "world");
    bar1("Hello", "world");

    fn bar<T, K>(x: T, y: K)
        where T: Clone,
              K: Clone + Debug {
        x.clone();
        y.clone();
        println!("{:?}", y);
    }


    trait ConvertTo<Output> {
        fn convert(&self) -> Output;
    }

    impl ConvertTo<i64> for i32 {
        fn convert(&self) -> i64 { *self as i64 }
    }

    // Can be called with T == i32.
    fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
        x.convert()
    }

    // Can be called with T == i64.
    fn inverse<T>(x: i32) -> T
        // This is using ConvertTo as if it were "ConvertTo<i64>".
        where i32: ConvertTo<T> {
        x.convert()
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Default methods

    trait Foo {
        fn is_valid(&self) -> bool;
        fn is_invalid(&self) -> bool { !self.is_valid() }
    }

    struct UseDefault;

    impl Foo for UseDefault {
        fn is_valid(&self) -> bool {
            println!("Called UseDefault.is_valid.");
            true
        }
    }

    struct OverrideDefault;

    impl Foo for OverrideDefault {
        fn is_valid(&self) -> bool {
            println!("Called OverrideDefault.is_valid.");
            true
        }

        fn is_invalid(&self) -> bool {
            println!("Called OverrideDefault.is_invalid!");
            true // Overrides the expected value of `is_invalid()`.
        }
    }

    let default = UseDefault;
    assert!(!default.is_invalid()); // Prints "Called UseDefault.is_valid."

    let over = OverrideDefault;
    assert!(over.is_invalid()); // Prints "Called OverrideDefault.is_invalid!"

    ////////////////////////////////////////////////////////////////////////////////
    // Inheritance

    trait Foo2 {
        fn foo(&self);
    }

    trait FooBar : Foo2 {
        fn foobar(&self);
    }

    struct Baz;

    impl Foo2 for Baz {
        fn foo(&self) { println!("foo"); }
    }

    impl FooBar for Baz {
        fn foobar(&self) { println!("foobar"); }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Deriving

    // #[derive(Debug)]
}
