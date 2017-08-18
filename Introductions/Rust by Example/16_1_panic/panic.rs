// https://rustbyexample.com/error/panic.html
// http://rust-lang-ja.org/rust-by-example/error/unwrap.html

fn give_princess(gift: &str) {
    // Princesses hate snakes, so we need to stop if she disapproves!
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    give_princess("teddy hear");
    give_princess("snake");
}
