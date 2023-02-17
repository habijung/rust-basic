fn main() {
    println!("Hello, world!");

    let mut a = 100;
    let b = 3.14;
    let c: u32 = 12345;
    let d: f32 = 3.14;
    let e: bool = true;

    a = a + 1;
    println!("{} and {}", a, d);

    const PI: f64 = 3.141592;

    let area = PI * 5.0 * 5.0;
    println!("{}", area);

    let a = "hello";
    println!("a: {}", a);
}
