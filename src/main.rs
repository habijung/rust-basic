fn main() {
    println!("Hello, world!");

    let arr: [i32; 3] = [1; 3];
    println!("{}", arr[0]);
    println!("{:?}", arr);

    let dat: (i32, char, bool) = (1, 'A', true);
    let a = dat.0;
    let b: char = dat.1;
    let c: bool = dat.2;

    let (a, b, c) = dat;
}
