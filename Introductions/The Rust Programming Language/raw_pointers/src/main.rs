// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/raw-pointers.html
#![allow(unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Raw Pointers

    let x = 5;
    let raw = &x as *const i32;

    let mut y = 10;
    let raw_mut = &mut y as *mut i32;

    let x = 5;
    let raw = &x as *const i32;

    // error[E0133]: dereference of raw pointer requires unsafe function or block
    // println!("raw points at {}", *raw);

    let points_at = unsafe { *raw };
    println!("raw points at {}", points_at);

    ////////////////////////////////////////////////////////////////////////////////
    // References and raw pointers

    // Explicit cast:
    let i: u32 = 1;
    let p_imm: *const u32 = &i as *const u32;

    // Implicit coercion:
    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m;

    unsafe {
        let ref_imm: &u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut;
    }
}
