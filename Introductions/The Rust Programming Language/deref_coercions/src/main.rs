// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/deref-coercions.html
#![allow(unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // `Deref` coercions

    use std::ops::Deref;

    struct DerefExample<T> {
        value: T,
    }

    impl<T> Deref for DerefExample<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.value
        }
    }

    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);


    fn foo(s: &str) {
        // Borrow a string for a second.
    }
    // String implements Deref<Target=str>.
    let owned = "Hello".to_string();
    // Therefore, this works:
    foo(&owned);


    use std::rc::Rc;

    fn foo2(s: &str) {
        // Borrow a string for a second.
    }
    // String implements Deref<Target=str>.
    let owned = "Hello".to_string();
    let counted = Rc::new(owned);

    // Therefore, this works:
    foo2(&counted);


    fn foo3(s: &[i32]) {
        // Borrow a slice for a second.
    }

    // Vec<T> implements Deref<Target=[T]>.
    let owned = vec![1, 2, 3];
    foo3(&owned);

    ////////////////////////////////////////////////////////////////////////////////
    // Deref and method calls

    struct Foo;

    impl Foo {
        fn foo(&self) { println!("Foo"); }
    }

    let f = &&Foo;
    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&f).foo();
}
