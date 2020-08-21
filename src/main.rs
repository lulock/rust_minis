use std::io;

fn main() {
    println!("Welcome ... What is your name?");

    let mut name = String::new();

    io::stdin().read_line(&mut name);
    welcome(&name);

    println!("Please insert a number:");
    let mut first_number = String::new();
 
    io::stdin().read_line(&mut first_number);
    let a:u32 = first_number.trim().parse().unwrap();

    println!("Please insert a second number:");
    let mut second_number = String::new();

    io::stdin().read_line(&mut second_number);
    let b:u32 = second_number.trim().parse().unwrap();

    let result = sum(a,b);
    println!("The sum of {} and {} is {}!", a, b, result);
}

fn welcome(first: &String){
    println!("\nHello {}", first);
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}
