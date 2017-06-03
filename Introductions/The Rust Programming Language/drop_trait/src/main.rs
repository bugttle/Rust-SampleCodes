// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/drop.html
#![allow(unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Drop

    struct HasDrop;

    impl Drop for HasDrop {
        fn drop(&mut self) {
            println!("Dropping!");
        }
    }

    {
        let x = HasDrop;
        // Do stuff.
    } // `x` goes out of scope here.


    struct Firework {
        strength: i32,
    }

    impl Drop for Firework {
        fn drop(&mut self) {
            println!("BOOM times {}!!!", self.strength);
        }
    }

    {
        let firecracker = Firework { strength: 1 };
        let tnt = Firework { strength: 100 };
    }
}
