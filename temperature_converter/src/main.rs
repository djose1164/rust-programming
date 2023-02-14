use std::io::Write;

fn celsius_to_f(degress: i32) -> i32 {
    (degress * 9) / 5 + 32
}

fn fahrenheit_to_c(degress: i32) -> i32 {
    (degress - 32) * 5 / 9
}

fn get_degrees() -> i32 {
    print!("Enter degress: ");
    std::io::stdout().flush().unwrap();

    let mut degress = String::new();
    std::io::stdin()
            .read_line(&mut degress)
            .expect("Failed to read line");

    let degress: i32 = degress
            .trim()
            .parse()
            .expect("Well, be sure you've typed in a number!");

    degress
}

fn main() {
    println!(
        "\t*-*-*-*-\tTempConv\t*-*-*-*\n\
            \tYour converter of F to C and viceversa!\n\n\
            Type in the number of the option:       \n\
            1. C to F                               \n\
            2. F to C               \n\
            3. exit"
    );
    std::io::stdout().flush().unwrap();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut opt = String::new();

        std::io::stdin()
            .read_line(&mut opt)
            .expect("Failed to read line");

        let opt: u16 = opt
            .trim()
            .parse()
            .expect("Well, be sure you've typed in a number!");

        if opt == 3 { break; }

        match opt {
            1 => {
                let degrees = get_degrees();
                println!("{degrees}°C to °F is: {}°F", celsius_to_f(degrees));
            },
            2 => {
                let degrees = get_degrees();
                println!("{degrees}°F to °C is: {}°C", fahrenheit_to_c(degrees));
            },
            _ => println!("Invalid option.")
        }
    }
}
