fn main() {
    let tup = (3, 5, "HOlalala");
    println!("{} {} {}", tup.0, tup.1, tup.2);
    // println!("{(tup.0)} {tup.2}"); //--- Do not work

    let (a, b, c) = tup;
    println!("a: {a} b: {b} c: {c}");
}
