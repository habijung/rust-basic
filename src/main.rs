struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

fn main() {
    println!("Hello, world!");

    let pt = Point { x: 2.0, y: 2.0 };

    println!("{}", pt.get_x());
    println!("{}", pt.get_y());
}
