mod shape;

pub mod usa {
    pub mod washington {
        pub mod seattle {
            pub fn trip() {
                println!("Trip to Seattle");
            }
        }
    }
}

use usa::washington::seattle;
use usa::washington::seattle as s;
use usa::washington::seattle::trip;

fn main() {
    println!("Hello, world!");

    // shape::rect::draw();

    // Full path
    usa::washington::seattle::trip();

    // Simple path
    seattle::trip();

    // Alias path
    s::trip();

    // Use directly
    trip();
}
