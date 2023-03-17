mod circle;

fn main() {
    println!("Hello, world!");

    let a = circle::area(5.0);
    let p = circle::perimeter(5.0);
    println!("{}, {}", a, p);
}
