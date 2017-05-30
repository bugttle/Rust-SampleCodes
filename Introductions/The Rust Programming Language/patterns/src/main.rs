// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/patterns.html
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Patterns

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 'x';
    let c = 'c';
    match c {
        x => println!("x: {} c: {}", x, c),
    }
    println!("x: {}", x);

    ////////////////////////////////////////////////////////////////////////////////
    // Multiple patterns

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Destructuring

    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, y } => println!("({},{})", x, y),
    }

    match origin {
        Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
    }

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    match origin {
        Point { y, .. } => println!("y is {}", y),
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Ignoring bindings

    let some_value: Result<i32, &'static str> = Err("There was an error");
    match some_value {
        Ok(value) => println!("got a value: {}", value),
        Err(_) => println!("an error occurred"),
    }

    fn coordinate() -> (i32, i32, i32) {
        (1, 2, 3) // Generate and return some sort of triple tuple.
    }
    let (x, _, z) = coordinate();

    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }

    let x = OptionalTuple::Value(5, -2, 3);

    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"),
        OptionalTuple::Missing => println!("No such luck."),
    }

    ////////////////////////////////////////////////////////////////////////////////
    // ref and ref mut

    let x = 5;
    match x {
        ref r => println!("Got a reference to {}", r),
    }

    let mut x = 5;
    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Ranges

    let x = 1;
    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("anything"),
    }

    let x = '💅';
    match x {
        'a' ... 'j' => println!("early letter"),
        'k' ... 'z' => println!("late letter"),
        _ => println!("something else"),
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Bindings

    let x = 1;
    match x {
        e @ 1 ... 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }


    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let mut x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }

    let x = 5;
    match x {
        e @ 1 ... 5 | e @ 8 ... 10 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Guards

    enum OptionalInt {
        Value(i32),
        Missing,
    }
    let x = OptionalInt::Value(5);
    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 if y => println!("yes"),
        _ => println!("no"),
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Mix and Match

    // match x {
    //     Foo { x: Some(ref name), y: None } => ...
    // }
}
