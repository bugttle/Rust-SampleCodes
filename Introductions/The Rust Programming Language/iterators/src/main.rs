// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/iterators.html
#![allow(unused_must_use, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Iterators

    for x in 0..10 {
        println!("{}", x);
    }

    let mut range = 0..10;

    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => { break }
        }
    }

    let nums = vec![1, 2, 3];
    for i in 0..nums.len() {
        println!("{}", nums[i]);
    }

    for num in &nums {
        println!("{}", num);
    }

    for num in &nums {
        println!("{}", *num);
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Consumers

    // error[E0282]: type annotations needed
    // let one_to_one_hundred = (1..101).collect();

    let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
    let one_to_one_hundred = (1..101).collect::<Vec<_>>();

    let greater_than_forty_two = (0..100).find(|x| *x > 42);
    match greater_than_forty_two {
        Some(_) => println!("Found a match!"),
        None => println!("No match found :("),
    }

    let sum = (1..4).fold(0, |sum, x| sum + x);


    ////////////////////////////////////////////////////////////////////////////////
    // Iterators

    let nums = 1..100;
    let nums = (1..100).collect::<Vec<i32>>();

    let nums = vec![1, 2, 3];
    for num in nums.iter() {
        println!("{}", num);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Iterator adaptors

    // warning: unused result which must be used: iterator adaptors are lazy and do nothing unless consumed
    (1..100).map(|x| x + 1);

    for i in (1..).take(5) {
        println!("{}", i);
    }

    for i in (1..100).filter(|&x| x % 2 == 0) {
        println!("{}", i);
    }

    (1..)
        .filter(|&x| x % 2 == 0)
        .filter(|&x| x % 3 == 0)
        .take(5)
        .collect::<Vec<i32>>();
}
