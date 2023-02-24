fn main() {
    println!("Hello, world!");

    let c = add(1, 2);
    println!("{}", c);
}

fn add(a: i32, b: i32) -> i32 {
    dbg!(a, b);
    return a + b;
}
