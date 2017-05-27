// https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/structs.html
#![allow(dead_code, unused_assignments, unused_mut, unused_variables)]

fn main() {
    ////////////////////////////////////////////////////////////////////////////////
    // Structs

    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 }; // origin: Point
    println!("The origin is at ({}, {})", origin.x, origin.y);

    let mut point = Point { x: 0, y: 0 };
    point.x = 5;
    println!("The point is at ({}, {})", point.x, point.y);

    let mut origin = Point { x: 0, y: 0 };
    point.x = 5;
    let point = point; // `point` is now immutable.
    // point.y = 6; // This causes an error. error: cannot assign to immutable field `point.y`

    ////////////////////////////////////////////////////////////////////////////////
    // Update syntax

    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut point = Point3d { x: 0, y: 0, z: 0 };
    point = Point3d { y: 1, .. point };

    let origin = Point3d { x: 0, y: 0, z: 0 };
    let point = Point3d { z: 1, x: 2, .. origin };

    ////////////////////////////////////////////////////////////////////////////////
    // Tuple structs

    struct ColorTuple(i32, i32, i32);
    struct PointTuple(i32, i32, i32);

    let black = ColorTuple(0, 0, 0);
    let origin = PointTuple(0, 0, 0);

    struct Color {
        red: i32,
        blue: i32,
        green: i32,
    }

    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }

    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);

    ////////////////////////////////////////////////////////////////////////////////
    // Tuple structs

    struct Electron {}; // Use empty braces...
    struct Proton;; // ...or just a semicolon.

    // Use the same notation when creating an instance.
    let x = Electron {};
    let y = Proton;
    // let z = Electron; // error: expected value, found struct `Electron`
}
