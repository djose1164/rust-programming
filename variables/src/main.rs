fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The values of x in the inner scope is: {x}");    
    }

    println!("The values of x is: {x}");
}
