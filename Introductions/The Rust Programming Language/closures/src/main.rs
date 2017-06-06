// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/closures.html
#![allow(dead_code, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Closures

    ////////////////////////////////////////////////////////////////////////////////
    // Syntax

    let plus_one = |x: i32| x + 1;
    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut result: i32 = x;

        result += 1;
        result += 1;

        result
    };
    assert_eq!(4, plus_two(2));

    let plus_one = |x: i32| -> i32 { x + 1 };
    assert_eq!(2, plus_one(1));


    fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
    let plus_one_v2 = |x: i32| -> i32 { x + 1 };
    let plus_one_v3 = |x: i32|          x + 1  ;

    ////////////////////////////////////////////////////////////////////////////////
    // Closures and their environment

    let num = 5;
    let plus_num = |x: i32| x + num;
    assert_eq!(10, plus_num(5));

    // let y = &mut num; // error: cannot borrow immutable local variable `num` as mutable

    let mut num = 5;
    {
        let plus_num = |x: i32| x + num;
    } // `plus_num` goes out of scope; borrow of `num` ends.
    let y = &mut num;

    let nums = vec![1, 2, 3];
    let takes_num = || nums;
    // println!("{:?}", nums); // error[E0382]: use of moved value: `nums`

    ////////////////////////////////////////////////////////////////////////////////
    // move closures

    let num = 5;
    let owns_num = move |x: i32| x + num;

    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(10, num);

    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(5, num);

    ////////////////////////////////////////////////////////////////////////////////
    // Closure implementation

    // pub trait Fn<Args> : FnMut<Args> {
    //     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    // }

    // pub trait FnMut<Args> : FnOnce<Args> {
    //     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    // }

    // pub trait FnOnce<Args> {
    //     type Output;
    //     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    // }

    ////////////////////////////////////////////////////////////////////////////////
    // Taking closures as arguments

    fn call_with_one<F>(some_closure: F) -> i32
        where F : Fn(i32) -> i32 {
        some_closure(1)
    }
    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);

    //fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
    //    some_closure(1)
    //}
    //let answer = call_with_one(&|x| x + 2);
    //assert_eq!(3, answer);

    ////////////////////////////////////////////////////////////////////////////////
    // Function pointers ans closures

    //fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
    //    some_closure(1)
    //}

    //fn add_one(i: i32) -> i32 {
    //    i + 1
    //}

    //let f = add_one;
    //let answer = call_with_one(&f);
    //asswert_eq!(2, answer);

    ////////////////////////////////////////////////////////////////////////////////
    // Returning closures

    // error[E0277]: the trait bound `std::ops::Fn(i32) -> i32: std::marker::Sized` is not satisfied
    // fn factory() -> (Fn(i32) -> i32) {
    //     let num = 5;
    //     |x| x + num
    // }

    // let f = factory();
    // let answer = f(1);
    // assert_eq!(6, answer);

    // error[E0106]: missing lifetime specifier
    // fn factory() -> &(Fn(i32) -> i32) {
    //     let num = 5;
    //     |x| x + num
    // }

    // let f = factory();
    // let answer = f(1);
    // assert_eq!(6, answer);

    // error[E0373]: closure may outlive the current function, but it borrows `num`, which is owned by the current functi
    // fn factory() -> Box<Fn(i32) -> i32> {
    //     let num = 5;
    //     Box::new(|x| x + num)
    // }
    // let f = factory();
    // let answer = f(1);
    // assert_eq!(6, answer);

    fn factory() -> Box<Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }
    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer);
}
