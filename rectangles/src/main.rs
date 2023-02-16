#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 10,
    };

    // println!("The area of the rectangle is {} square pixels.", rec.are());
    if rec.width() {
        println!("The rectangle has a nonzero width; it is {}", rec.width);
    }
}
