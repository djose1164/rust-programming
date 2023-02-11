fn another_function(x: i32) {
    println!("The values of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn main() {
    println!("Hello, world!");

    another_function(3);
    print_labeled_measurement(6, 'd');
}
