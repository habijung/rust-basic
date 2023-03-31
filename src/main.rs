trait Draw {
    fn draw(&self, x: i32, y: i32);
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Draw for Rectangle {
    fn draw(&self, x: i32, y: i32) {
        let x2 = x + self.width;
        let y2 = y + self.height;
        println!("Rect({}, {} ~ {}, {})", x, y, x2, y2);
    }
}

struct Circle {
    radius: i32,
}

impl Draw for Circle {
    fn draw(&self, x: i32, y: i32) {
        println!("Circle({}, {}, {})", x, y, self.radius);
    }
}

fn draw_it(item: impl Draw, x: i32, y: i32) {
    item.draw(x, y);
}

fn draw_it_bound<T: Draw>(item: T, x: i32, y: i32) {
    item.draw(x, y);
}

fn draw_basic_circle() -> impl Draw {
    Circle { radius: 1 }
}

fn main() {
    println!("Hello, world!");

    let rect = Rectangle { width: 20, height: 20 };
    let rect_bound = Rectangle { width: 30, height: 30 };
    let circle = Circle { radius: 5 };

    draw_it(rect, 1, 1);
    draw_it(circle, 2, 2);
    draw_basic_circle().draw(3, 3);
    draw_it_bound(rect_bound, 4, 4);
}
