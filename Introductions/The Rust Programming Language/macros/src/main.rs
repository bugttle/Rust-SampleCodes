// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/macros.html
#![allow(dead_code, unused_must_use, unused_variables)]
// #![feature(trace_macros)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Macros

    let x: Vec<u32> = vec![1, 2, 3];

    let x: Vec<u32> = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    };

    ////////////////////////////////////////////////////////////////////////////////
    // Defining a macro

    macro_rules! vec_macro {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Matching

    macro_rules! foo {
        (x => $e:expr) => (println!("mode X: {}", $e));
        (y => $e:expr) => (println!("mode Y: {}", $e));
    }

    foo!(y => 3);
    // foo!(z => 3); // error: no rules expected the token `z`

    ////////////////////////////////////////////////////////////////////////////////
    // Repetition

    macro_rules! o_O {
        (
            $(
                $x:expr; [ $( $y:expr ),* ]
            );*
        ) => {
            &[ $($( $x + $y ),*),* ]
        }
    }

    let a: &[i32]
        = o_O!(10; [1, 2, 3];
               20; [4, 5, 6]);
    assert_eq!(a, [11, 12, 13, 24, 25, 26]);

    ////////////////////////////////////////////////////////////////////////////////
    // Hygiene

    macro_rules! five_times {
        ($x:expr) => (5 * $x);
    }

    assert_eq!(25, five_times!(2 + 3));

    fn get_log_state() -> i32 { 3 }
    macro_rules! log {
        ($msg:expr) => {{
            let state: i32 = get_log_state();
            if state > 0 {
                println!("log({}): {}", state, $msg);
            }
        }};
    }

    let state: &str = "reticulating splines";
    log!(state);


    // macro_rules! foo {
    //     () => (let x = 3);
    // }
    //
    // foo!();
    // println!("{}", x); error[E0277]: the trait bound `std::vec::Vec<u32>: std::fmt::Display` is not satisfied

    macro_rules! foo2 {
        // if the function name is `x`, it will get the following error.
        // error: expected function, found `std::vec::Vec<u32>`
        // Because,
        //   1st: processing macro
        //   2nd: `x: Vec<u32>` will be shadowed
        () => (fn x2() { }); 
    }
    foo2!();
    x2();

    ////////////////////////////////////////////////////////////////////////////////
    // Recursive macros

    macro_rules! write_html {
        ($w:expr, ) => (());

        ($w:expr, $e:tt) => (write!($w, "{}", $e));

        ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
            write!($w, "<{}>", stringify!($tag));
            write_html!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag));
            write_html!($w, $($rest)*);
        }};
    }

    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out,
        html[
            head[title["Macros guide"]]
            body[h1["Macros are the best!"]]
        ]);

    assert_eq!(out,
        "<html><head><title>Macros guide</title></head>\
         <body><h1>Macros are the best!</h1></body></html>");

    ////////////////////////////////////////////////////////////////////////////////
    // Debugging macro code

    // `rustc --pretty expanded`
    // `rustc --pretty expanded,hygiene`

    // `log_syntax!(...)`

    // `trace_macros!(true)`
    // `trace_macros!(false)`

    ////////////////////////////////////////////////////////////////////////////////
    // Syntactic requirements

    // `ident`: an identifier. Examples: x; foo.
    // `path`: a qualified name. Example: T::SpecialA.
    // `expr`: an expression. Examples: 2 + 2; if true { 1 } else { 2 }; f(42).
    // `ty`: a type. Examples: i32; Vec<(char, String)>; &T.
    // `pat`: a pattern. Examples: Some(t); (17, 'a'); _.
    // `stmt`: a single statement. Example: let x = 3.
    // `block`: a brace-delimited sequence of statements and optionally an expression. Example: { log(error, "hi"); return 12; }.
    // `item`: an item. Examples: fn foo() { }; struct Bar;.
    // `meta`: a "meta item", as found in attributes. Example: cfg(target_os = "windows").
    // `tt`: a single token tree.

    ////////////////////////////////////////////////////////////////////////////////
    // Scoping and macro import/export

    macro_rules! m1 { () => (()) }

    // Visible here: `m1`.

    mod foo {
        // Visible here: `m1`.

        #[macro_export]
        macro_rules! m2 { () => (()) }

        // Visible here: `m1`, `m2`.
    }

    // Visible here: `m1`.

    macro_rules! m3 { () => (()) }

    // Visible here: `m1`, `m3`.

    #[macro_use]
    mod bar {
        // Visible here: `m1`, `m3`.

        macro_rules! m4 { () => (()) }

        // Visible here: `m1`, `m3`, `m4`.
    }

    // Visible here: `m1`, `m3`, `m4`.

    ////////////////////////////////////////////////////////////////////////////////
    // The variable `$crate`

    pub fn increment(x: u32) -> u32 {
        x + 1
    }

    #[macro_export]
    macro_rules! inc_a {
        ($x:expr) => ( ::increment($x) )
    }

    #[macro_export]
    macro_rules! inc_b {
        ($x:expr) => ( ::mylib::increment($x) )
    }

    #[macro_export]
    macro_rules! inc {
        ($x:expr) => ( $crate::increment($x) )
    }

    ////////////////////////////////////////////////////////////////////////////////
    // The deep end

    // uqtimes: I think the macro is not a cyclic tag system.
    macro_rules! bct {
        // cmd 0: d ... => ...
        (0, $($ps:tt),* ; $_d:tt)
            => (bct!($($ps),*, 0 ; ));
        (0, $($ps:tt),* ; $_d:tt, $($ds:tt),*)
            => (bct!($($ps),*, 0 ; $($ds),*));

        // cmd 1p: 1 ... => 1 ... p
        (1, $p:tt, $($ps:tt),* ; 1)
            => (bct!($($ps),*, 1, $p ; 1, $p));
        (1, $p:tt, $($ps:tt),* ; 1, $($ds:tt),*)
            => (bct!($($ps),*, 1, $p ; 1, $($ds),*, $p));

        // cmd 1p: 0 ... => 0 ...
        (1, $p:tt, $($ps:tt),* ; $($ds:tt),*)
            => (bct!($($ps),*, 1, $p ; $($ds),*));

        // Halt on empty data string:
        ( $($ps:tt),* ; )
            => (());
    }

    // trace_macros!(true);
    // bct!(1, 1, 1, 0, 0 ; 1, 0, 1); // error: recursion limit reached while expanding the macro `bct`
    // trace_macros!(false);

    macro_rules! bct_exercise {
        // cmd 0: d ... => ...
        (0 $(, $ps:tt)* ; $_d:tt $(, $ds:tt)*)
            => (bct_exercise!($($ps),*, 0 ; $($ds),*));

        // cmd 1p: 1 ... => ... p
        (1 $(, $ps:tt)* ; $_d:tt $(, $ds:tt)*)
            => (bct_exercise!($($ps),*, 1 ; $($ds, )* 1));

        // Halt on empty data string:
        ( $($ps:tt),* ; )
            => (());
    }

    // trace_macros!(true);
    // bct_exercise!(1, 1, 1, 0, 0 ; 1, 0, 1);
    // trace_macros!(false);

    // Result:
    //   bct_exercise! { 1 , 1 , 1 , 0 , 0 ; 1 , 0 , 1 }
    //   bct_exercise! { 1 , 1 , 0 , 0 , 1 ; 0 , 1 , 1 }
    //   bct_exercise! { 1 , 0 , 0 , 1 , 1 ; 1 , 1 , 1 }
    //   bct_exercise! { 0 , 0 , 1 , 1 , 1 ; 1 , 1 , 1 }
    //   bct_exercise! { 0 , 1 , 1 , 1 , 0 ; 1 , 1 }
    //   bct_exercise! { 1 , 1 , 1 , 0 , 0 ; 1 }
    //   bct_exercise! { 1 , 1 , 0 , 0 , 1 ; 1 }
    //   bct_exercise! { 1 , 0 , 0 , 1 , 1 ; 1 }
    //   bct_exercise! { 0 , 0 , 1 , 1 , 1 ; 1 }
    //   bct_exercise! { 0 , 1 , 1 , 1 , 0 ; }

    ////////////////////////////////////////////////////////////////////////////////
    // Common macros

    //// panic!
    // panic!("oh no!");

    //// vec!
    let v = vec![1, 2, 3, 4, 5];
    let v = vec![0; 100];

    //// assert! and assert_eq!

    // A-ok!
    assert!(true);
    assert_eq!(5, 3 + 2);

    // Nope :(
    // assert!(5 < 3);
    // assert_eq!(5, 3);

    //// try!
    use std::fs::File;

    fn foo() -> std::io::Result<()> {
        let f = try!(File::create("foo.txt"));

        Ok(())
    }

    // cleaner than doing this:
    fn foo2() -> std::io::Result<()> {
        let f = File::create("foo.txt");

        let f = match f {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    //// unreachable!

    if false {
        unreachable!();
    }

    let x: Option<i32> = None;
    match x {
        Some(_) => unreachable!(),
        None => println!("I know x is None!"),
    }

    //// unimplemented!
}
