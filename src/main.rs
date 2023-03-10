struct Person {
    id: i32,
    name: String,
    active: bool
}

impl Person {
    fn new(id: i32, name: String) -> Person {
        Person{ id: id, name: name, active: true }
    }

    fn display(&self) {
        if self.active {
            println!("{}: {}", self.id, self.name);
        }
        else {
            println!("{}: inactive", self.id);
        }
    }

    fn set_active(&mut self, is_active: bool) {
        self.active = is_active;
    }
}

fn main() {
    println!("Hello, world!");

    let mut p = Person::new(101, String::from("Tom"));
    p.set_active(false);
    p.display();
}
