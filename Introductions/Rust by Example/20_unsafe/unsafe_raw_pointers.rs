// https://rustbyexample.com/unsafe.html
// http://rust-lang-ja.org/rust-by-example/unsafe.html

fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}
