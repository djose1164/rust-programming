#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 10
    };

    println!("rec is {:#?}", rec);
}
