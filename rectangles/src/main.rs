fn area(w: u32, h: u32) -> u32 {
    w * h
}

fn main() {
    let width = 30;
    let height = 10;

    println!(
        "The area of the rectangles is {} square pixels.",
        area(width, height)
    );
}
