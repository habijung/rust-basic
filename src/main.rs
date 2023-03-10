struct Member {
    fname: String,
    lname: String,
    age: u16,
    active: bool
}

fn main() {
    println!("Hello, world!");

    // 구조체 초기화
    let mem1 = Member {
        active: true,
        fname: String::from("Tom"),
        lname: String::from("Lee"),
        age: 35
    };

    // 구조체 필드 읽기
    println!("{}: {}", mem1.fname, mem1.active);
}
