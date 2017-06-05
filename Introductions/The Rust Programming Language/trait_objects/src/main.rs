// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/trait-objects.html
#![allow(unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Trait Objects

    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Foo for String {
        fn method(&self) -> String {
            format!("string: {}", *self)
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Static dispatch

    fn do_something<T: Foo>(x: T) {
        x.method();
    }

    let x = 5u8;
    let y = "Hello".to_string();

    do_something(x);
    do_something(y);


    fn do_something_u8(x: u8) {
        x.method();
    }
    fn do_something_string(x: String) {
        x.method();
    }

    let x = 5u8;
    let y = "Hello".to_string();

    do_something_u8(x);
    do_something_string(y);

    ////////////////////////////////////////////////////////////////////////////////
    // Dynamic dispatch

    fn do_something2(x: &Foo) {
        x.method();
    }

    let x = 5u8;
    do_something2(&x as &Foo);

    let x = "Hello".to_string();
    do_something2(&x);

    ////////////////////////////////////////////////////////////////////////////////
    // Representation

    // pub struct TraitObject {
    //     pub data: *mut (),
    //     pub vtable: *mut (),
    // }

    // struct FooVtable {
    //     destructor: fn(*mut ()),
    //     size: usize,
    //     align: usize,
    //     method: fn(*const ()) -> String,
    // }

    //// u8:

    // fn call_method_on_u8(x: *const()) -> String {
    //     // The compiler guarantees that this function is only called
    //     // with `x` pointing to a u8.
    //     let byte: &u8 = unsafe { &*(x as *const u8) };

    //     byte.method()
    // }

    // static Foo_for_u8_vtable: FooVtable = FooVtable {
    //     destructor: /* compiler magic */,
    //     size: 1,
    //     align: 1,

    //     // Call to a function pointer:
    //     method: call_method_on_u8 as fn(*const ()) -> String,
    // };


    // // String:

    // fn call_method_on_String(x: *const ()) -> String {
    //     // The compiler guarantees that this function is only called
    //     // with `x` pointing to a String.
    //     let string: &String = unsafe { &*(x as *const String) };

    //     string.method()
    // }

    // static Foo_for_String_vtable: FooVtable = FooVtable {
    //     destructor: /* compiler magic */,
    //     // Values for a 64-bit compiler, halve them for 32-bit ones.
    //     size: 24,
    //     align: 8,

    //     method: call_method_on_String as fn(*const ()) -> String,
    // };

    // let a: String = "foo".to_string();
    // let x: u8 = 1;

    // // let b: &Foo = &a;
    // let b = TraitObject {
    //     // Store the data:
    //     data: &a,
    //     // Store the methods:
    //     vtable: &Foo_for_String_vtable
    // };

    // // let y: &Foo = x;
    // let y = TraitObject {
    //     // Store the data:
    //     data: &x,
    //     // Store the methods:
    //     vtable: &Foo_for_u8_vtable
    // };

    // // b.method();
    // (b.vtable.method)(b.data);

    // // y.method();
    // (y.vtable.method)(y.data);

    ////////////////////////////////////////////////////////////////////////////////
    // Object Safety

    let v = vec![1, 2, 3];
    // let o = &v as &Clone; // error[E0038]: the trait `std::clone::Clone` cannot be made into an object
}
