struct Person {
    id: i32,
    name: String,
    active: bool
}

impl Person {
    fn new(id: i32, name: String) -> Person {
        Person{ id: id, name: name, active: true }
    }
}

fn main() {
    println!("Hello, world!");

    let p = Person::new(101, String::from("Tom"));

    println!("{}: {}", p.id, p.name);
}
