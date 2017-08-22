// https://rustbyexample.com/error/multiple_error_types.html
// http://rust-lang-ja.org/rust-by-example/error/option_with_result.html

// fn double_first(vec: Vec<&str>) -> i32 {
//     let first = vec.first().unwrap(); // Generate error 1
//     2 * first.parse::<i32>().unwrap() // Generate error 2
// }
// 
// fn main() {
//     let empty = vec![];
//     let strings = vec!["tofu", "93", "18"];
// 
//     println!("The first doubled is {}", double_first(empty));
//     // Error 1: the input vector is empty
// 
//     println!("The first doubled is {}", double_first(strings));
//     // Error 2: the element doesn't parse to a number
// }

// Use `String` as our error type
type Result<T> = std::result::Result<T, String>;

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // Convert the `Option` to a `Result` if there is a value.
        // Otherwise, provide an `Err` containing this `String`.
        .ok_or("Please use a vector with at least one element.".to_owned())
        .and_then(|s| s.parse::<i32>()
                       // Map any errors that `parse` yields to `String`.
                       .map_err(|e| e.to_string())
                       // `Result<T, String>` is the new return type,
                       // and we can now double the number inside.
                       .map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(empty));
    print(double_first(strings));
}
