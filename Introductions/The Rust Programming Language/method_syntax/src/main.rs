// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/method-syntax.html
#![allow(dead_code, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Method calls

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());


    impl Circle {
        fn reference(&self) {
            println!("taking self by reference!");
        }

        fn mutable_reference(&mut self) {
            println!("taking self by mutable reference!");
        }

        fn takes_ownership(self) {
            println!("taking ownership of self!");
        }
    }

    //impl Circle {
    //    fn reference(&self) {
    //        println!("taking self by reference!");
    //    }
    //}

    //impl Circle {
    //    fn mutable_reference(&mut self) {
    //        println!("taking self by mutable reference!");
    //    }
    //}

    //impl Circle {
    //    fn takes_ownership(self) {
    //        println!("taking ownership of self!");
    //    }
    //}

    ////////////////////////////////////////////////////////////////////////////////
    // Chaining method calls

    impl Circle {
        fn area2(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }

        fn grow(&self, increment: f64) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius + increment }
        }
    }

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area2());

    let d = c.grow(2.0).area2();
    println!("{}", d);

    ////////////////////////////////////////////////////////////////////////////////
    // Associated functions

    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }
    }

    let c = Circle::new(0.0, 0.0, 2.0);

    ////////////////////////////////////////////////////////////////////////////////
    // Builder Pattern

    struct CircleBuilder {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl CircleBuilder {
        fn new() -> CircleBuilder {
            CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
        }

        fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.x = coordinate;
            self
        }

        fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.y = coordinate;
            self
        }

        fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
            self.radius = radius;
            self
        }

        fn finalize(&self) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius }
        }
    }

    let c = CircleBuilder::new()
                .x(1.0)
                .y(2.0)
                .radius(2.0)
                .finalize();

    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);
}
