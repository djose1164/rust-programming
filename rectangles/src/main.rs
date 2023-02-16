fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn main() {
    let rec = (30, 10);

    println!(
        "The area of the rectangles is {} square pixels.",
        area(rec)
    );
}
