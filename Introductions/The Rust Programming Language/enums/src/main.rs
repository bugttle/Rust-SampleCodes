// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/enums.html
#![allow(dead_code, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Enums

    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    let x: Message = Message::Move { x: 3, y: 4 };

    enum BoardGameTurn {
        Move { squares: i32 },
        Pass,
    }

    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

    fn process_color_change(msg: Message) {
        // let Message::ChangeColor(r, g, b) = msg; // error[E0005]: refutable pattern in local binding: `Quit` not covered
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Constructors as functions

    let m = Message::Write("Hello, world".to_string());
    fn foo(x: String) -> Message {
        Message::Write(x)
    }
    let x = foo("Hello, world".to_string());

    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
}
