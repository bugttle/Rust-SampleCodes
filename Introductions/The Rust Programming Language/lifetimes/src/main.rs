// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/lifetimes.html
#![allow(dead_code, unused_variables)]

struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // In structs

    let y = &5; // This is the same as `let _y = 5; let y = &_y;`.
    let f = Foo { x: y };
    println!("{}", f.x);

    ////////////////////////////////////////////////////////////////////////////////
    // Multiple lifetimes

    fn x_or_y<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        "STRING"
    }

    ////////////////////////////////////////////////////////////////////////////////
    // 'static

    let x: &'static str = "Hello, world.";

    static FOO: i32 = 5;
    let x: &'static i32 = &FOO;

    ////////////////////////////////////////////////////////////////////////////////
    // Examples

    // fn print(s: &str); // elided
    // fn print<'a>(s: &'a str); // expanded

    // fn debug(lvl: u32, s: &str); // elided
    // fn debug<'a>(lvl: u32, s: &'a str); // expanded

    // 前述の例では`lvl`はライフタイムを必要としません。なぜなら、それは参照（`&`）
    // ではないからです。（参照を含む`struct`のような）参照に関係するものだけがライ
    // フタイムを必要とします。

    // fn substr(s: &str, until: u32) -> &str; // elided
    // fn substr<'a>(s: &'a str, until: u32) -> &'a str; // expanded

    // fn get_str() -> &str; // ILLEGAL, no input

    // fn frob(s: &str, t: &str) -> &str; // ILLEGAL, two inputs
    // fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // Expanded: Output lifetime is ambiguous

    // fn get_mut(&mut self) -> &mut T; // elided
    // fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded

    // fn args<T:ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided
    // fn args<'a, 'b, T:ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

    // fn new(buf: &mut [u8]) -> BufWriter; // elided
    // fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded
}
