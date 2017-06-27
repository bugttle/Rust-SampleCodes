// https://rust-lang-ja.github.io/the-rust-programming-language-ja/2.6/book/ffi.html
#![allow(dead_code, private_no_mangle_fns, unused_imports)]

////////////////////////////////////////////////////////////////////////////////
// Foreign Function Interface

extern crate libc;
use libc::{c_int, size_t};

use std::ffi::CString;
use std::ptr;
use std::thread;
use std::panic::catch_unwind;

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Calling foreign functions

    // $ brew install snappy
    // $ LIBRARY_PATH=/usr/local/lib cargo build

    #[link(name = "snappy")]
    extern {
        fn snappy_compress(input: *const u8,
                           input_length: size_t,
                           compressed: *mut u8,
                           compressed_length: *mut size_t) -> c_int;
        fn snappy_uncompress(compressed: *const u8,
                             compressed_length: size_t,
                             uncompressed: *mut u8,
                             uncompressed_length: *mut size_t) -> c_int;
        fn snappy_max_compressed_length(source_length: size_t) -> size_t;
        fn snappy_uncompressed_length(compressed: *const u8,
                                      compressed_length: size_t,
                                      result: *mut size_t) -> c_int;
        fn snappy_validate_compressed_buffer(compressed: *const u8,
                                             compressed_length: size_t) -> c_int;
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Creating a safe interface

    pub fn validate_compressed_buffer(src: &[u8]) -> bool {
        unsafe {
            snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0
        }
    }

    pub fn compress(src: &[u8]) -> Vec<u8> {
        unsafe {
            let srclen = src.len() as size_t;
            let psrc = src.as_ptr();

            let mut dstlen = snappy_max_compressed_length(srclen);
            let mut dst = Vec::with_capacity(dstlen as usize);
            let pdst = dst.as_mut_ptr();

            snappy_compress(psrc, srclen, pdst, &mut dstlen);
            dst.set_len(dstlen as usize);
            dst
        }
    }

    pub fn uncompress(src: &[u8]) -> Option<Vec<u8>> {
        unsafe {
            let srclen = src.len() as size_t;
            let psrc = src.as_ptr();

            let mut dstlen: size_t = 0;
            snappy_uncompressed_length(psrc, srclen, &mut dstlen);

            let mut dst = Vec::with_capacity(dstlen as usize);
            let pdst = dst.as_mut_ptr();

            if snappy_uncompress(psrc, srclen, pdst, &mut dstlen) == 0 {
                dst.set_len(dstlen as usize);
                Some(dst)
            } else {
                None // SNAPPY_INVALID_INPUT
            }
        }
    }

    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);


    ////////////////////////////////////////////////////////////////////////////////
    // Callbacks from C code to Rust functions

    // $ cd extlib
    // $ gcc -Wall -c extlib.c
    // $ ar r libextlib.a extlib.o
    // LIBRARY_PATH=/usr/local/lib:./extlib cargo build

    extern fn callback1(a: i32) {
        println!("I'm called from C with value {0}", a);
    }

    #[link(name = "extlib")]
    extern {
        fn register_callback1(cb: extern fn(i32)) -> i32;
        fn trigger_callback1();
    }

    unsafe {
        register_callback1(callback1);
        trigger_callback1(); // Triggers the callback.
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Targeting callbacks to Rust objects

    // $ cd extlib
    // $ gcc -Wall -c extlib.c
    // $ ar r libextlib.a extlib.o
    // LIBRARY_PATH=/usr/local/lib:./extlib cargo build

    #[repr(C)]
    struct RustObject {
        a: i32,
        // Other members...
    }

    extern "C" fn callback2(target: *mut RustObject, a: i32) {
        println!("I'm called from C with value {0}", a);
        unsafe {
            // Update the value in RustObject with the value received from the callback:
            (*target).a = a;
        }
    }

    #[link(name = "extlib")]
    extern {
        fn register_callback2(target: *mut RustObject,
                              cb: extern fn(*mut RustObject, i32)) -> i32;
        fn trigger_callback2();
    }

    // Create the object that will be refrenced in the callback:
    let mut rust_object = Box::new(RustObject { a: 5 });

    unsafe {
        register_callback2(&mut *rust_object, callback2);
        trigger_callback2();
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Linking

    // #[link(name = "readline")]
    // #[link(name = "my_build_dependency", kind = "static")]
    // #[link(name = "CoreFoundation", kind = "framework")]


    ////////////////////////////////////////////////////////////////////////////////
    // Unsafe blocks

    unsafe fn kaboom(ptr: *const i32) -> i32 { *ptr }


    ////////////////////////////////////////////////////////////////////////////////
    // Accessing foreign globals

    // $ brew install readline
    // $ brew link --force readline
    // LIBRARY_PATH=/usr/local/lib:./extlib cargo build

    #[link(name = "readline")]
    extern {
        static rl_readline_version: libc::c_int;
    }

    println!("You have readline version {} installed.",
             unsafe { rl_readline_version as i32 });


    #[link(name = "readline")]
    extern {
        static mut rl_prompt: *const libc::c_char;
    }

    let prompt = CString::new("[my-awesome-shell] $").unwrap();
    unsafe {
        rl_prompt = prompt.as_ptr();

        println!("{:?}", rl_prompt);

        rl_prompt = ptr::null();
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Foreign calling conventions

    #[cfg(all(target_os = "win32", target_arch = "x86"))]
    #[link(name = "kernel32")]
    #[allow(non_snake_case)]
    extern "stdcall" {
        fn SetEnvironmentVariableA(n: *const u8, v: *const u8) -> libc::c_int;
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Calling Rust code from C

    #[no_mangle]
    pub extern fn hello_rust() -> *const u8 {
        "Hello, world!\0".as_ptr()
    }


    ////////////////////////////////////////////////////////////////////////////////
    // FFI and panics

    #[no_mangle]
    pub extern fn oh_no() -> i32 {
        let result = catch_unwind(|| {
            panic!("Oops!");
        });
        match result {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }


    ////////////////////////////////////////////////////////////////////////////////
    // Representing opaque structs

    extern "C" {
        pub fn foo(arg: *mut libc::c_void);
        pub fn bar(arg: *mut libc::c_void);
    }

    pub enum Foo {}
    pub enum Bar {}

    extern "C" {
        pub fn foo_opaque(arg: *mut Foo);
        pub fn bar_opaque(arg: *mut Bar);
    }
}
