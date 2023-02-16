#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /*fn area(&self) -> u32 {
        self.width * self.height
    }*/

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 10,
    };

    let rec2 = Rectangle {
        width: 29,
        height: 9,
    };

    let rec3 = Rectangle {
        width: 20,
        height: 40,
    };

    println!("Can rec hold rec2? {}", rec.can_hold(&rec2));
    println!("Can rec hold rec3? {}", rec.can_hold(&rec3));

    let square1 = Rectangle::square(5);

    println!("Can rec hold square1? {}", rec.can_hold(&square1));
}
