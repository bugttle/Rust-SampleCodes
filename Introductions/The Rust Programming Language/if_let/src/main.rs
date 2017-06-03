// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/if-let.html

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // if let

    let option = Some(5);
    fn foo(x: i32) {
        println!("foo: {}", x);
    }
    fn bar() {
        println!("bar");
    }

    match option {
        Some(x) => foo(x),
        None => {}
    }

    if option.is_some() {
        let x = option.unwrap();
        foo(x);
    }

    if let Some(x) = option {
        foo(x);
    } else {
        bar();
    }

    ////////////////////////////////////////////////////////////////////////////////
    // while let

    let mut v = vec![1, 3, 5, 7, 11];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    let mut v = vec![1, 3, 5, 7, 11];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}
