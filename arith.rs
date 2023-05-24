use std::io;

fn main() {
    println!("Enter two numbers:");

    let mut num1 = String::new();
    let mut num2 = String::new();

    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read num1");

    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read num2");

    let num1: f64 = num1.trim().parse().expect("Invalid input for num1");
    let num2: f64 = num2.trim().parse().expect("Invalid input for num2");

    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;
    let quotient = num1 / num2;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}
