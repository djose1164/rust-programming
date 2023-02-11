fn main() {
    /* Lists must, unlike to tuples, have the same type*/
    let list = [1, 2, 3];
    println!("list: {} {} {}", list[0], list[1], list[2]);

    let a: [f64; 4] = [1./1., 1./2., 1./3., 1./4.];
    println!("{} {} {} {}", a[0], a[1], a[2], a[3]);

    let b = [5; 10];
    println!("b[0]: {} b[9]: {}", b[0], b[9])
}
