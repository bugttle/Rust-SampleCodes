// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/ufcs.html

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Universal Function Call Syntax

    trait Foo {
        fn f(&self);
    }

    trait Bar {
        fn f(&self);
    }

    struct Baz;

    impl Foo for Baz {
        fn f(&self) { println!("Baz's impl of Foo"); }
    }

    impl Bar for Baz {
        fn f(&self) { println!("Baz's impl of Bar"); }
    }

    let b = Baz;
    // b.f(); // error[E0034]: multiple applicable items in scope

    Foo::f(&b);
    Bar::f(&b);

    ////////////////////////////////////////////////////////////////////////////////
    // Angle-bracket Form

    trait Foo2 {
        fn foo() -> i32;
    }

    struct Bar2;

    impl Bar2 {
        fn foo() -> i32 {
            20
        }
    }

    impl Foo2 for Bar2 {
        fn foo() -> i32 {
            10
        }
    }

    assert_eq!(10, <Bar2 as Foo2>::foo());
    assert_eq!(20, Bar2::foo());
}
