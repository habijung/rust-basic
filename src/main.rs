mod circle {
    const PI: f64 = 3.14;

    pub fn perimeter(radius: f64) -> f64 {
        2.0 * PI * radius
    }

    pub fn area(radius: f64) -> f64 {
        PI * radius * radius
    }
}

fn main() {
    println!("Hello, world!");

    let a = circle::area(5.0);
    let p = circle::perimeter(5.0);
    println!("{}, {}", a, p);
}
