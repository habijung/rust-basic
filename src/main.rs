fn main() {
    println!("Hello, world!");

    let s = String::from("Hello");
    // let s: String = "hello".to_owned();

    println!("{}", s);

    let mut s2 = String::new(); // empty string
    s2.push('H');
    s2.push('i');
    s2.push_str(" ASDF"); // 문자열 추가

    println!("{}", s2);

    s2 = s2.replace("Hi", "Hello");

    println!("{}", s2);

    let s3 = String::from("Hello World");

    if s3.contains(" ") {
        for w in s3.split_whitespace() {
            println!("{}", w);
        }
    }
}
