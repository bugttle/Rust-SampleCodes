// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/match.html
#![allow(dead_code, non_shorthand_field_patterns, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Match

    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let number = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };

    ////////////////////////////////////////////////////////////////////////////////
    // Matching on enums

    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    fn quit() { /* ... */ }
    fn change_color(r: i32, g: i32, b: i32) { /* ... */ }
    fn move_cursor(x: i32, y: i32) { /* ... */ }

    fn process_message(msg: Message) {
        match msg {
            Message::Quit => quit(),
            Message::ChangeColor(r, g, b) => change_color(r, g, b),
            Message::Move { x: x, y: y } => move_cursor(x, y),
            Message::Write(s) => println!("{}", s),
        };
    }
}
