struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 10
    };

    println!(
        "The area of the rectangles is {} square pixels.",
        area(&rec)
    );
}
