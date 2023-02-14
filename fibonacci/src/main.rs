use std::io::{self, Write};
fn main() {
    println!("***The Fibo generator***");
    print!("Enter a number (be careful, this can take some time): ");
    io::stdout().flush().unwrap();

    let mut number = String::new();
    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line.");

    let number: i32 = number
        .trim()
        .parse()
        .expect("It doesn't look like a number.");

    match number {
        0 => println!("0"),
        1 => println!("1"),
        _ => {
            let mut counter = 0;
            let mut previous = 0;
            let mut current = 1;
            print!("0, 1, ");
            while counter != number {
                let tmp = current;
                current = previous + current;
                previous = tmp;
                if counter == number - 1 {
                    print!("{current}.");
                } else {
                    print!("{current}, ")
                }
                counter += 1;
            }
        }
    }
}
