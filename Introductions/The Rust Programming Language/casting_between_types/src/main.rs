// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/casting-between-types.html
#![allow(unused_unsafe, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Casting Between Types

    ////////////////////////////////////////////////////////////////////////////////
    // Coercion

    // `&mut T` to `&T`
    // `*mut T` to `*const T`
    // `&T` to `*const T`
    // `&mut T` to `*mut T`

    ////////////////////////////////////////////////////////////////////////////////
    // as

    let x: i32 = 5;
    let y = x as i64;

    ////////////////////////////////////////////////////////////////////////////////
    // Numeric casts

    let one = true as u8;
    let at_sign = 64 as char;
    let two_hundred = -56i8 as u8;

    ////////////////////////////////////////////////////////////////////////////////
    // Pointer casts

    let a = 300 as *const char; // `a` is a pointer to location 300.
    let b = a as u32;

    ////////////////////////////////////////////////////////////////////////////////
    // transmute

    let a = [0u8, 0u8, 0u8, 0u8];
    // let b = a as u32; // Four u8s makes a u32. // error: non-scalar cast: `[u8; 4]` as `u32`

    use std::mem;

    unsafe {
        let a = [0u8, 0u8, 0u8, 0u8];
        let b = mem::transmute::<[u8; 4], u32>(a);
    }

    unsafe {
        let a = [0u8, 0u8, 0u8, 0u8];
        // error[E0512]: transmute called with differently sized types: [u8; 4] (32 bits) to u64 (64 bits)
        // let b = mem::transmute::<[u8; 4], u64>(a);
    }
}
