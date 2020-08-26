use std::io;
use std::process;

fn main() {
    println!("Welcome ... What is your name?");

    let mut name = String::new();

    io::stdin().read_line(&mut name).unwrap();
    welcome(&name);
    
    loop {
        println!("Please insert a number:");
        let a = read_user_input();

        println!("Please insert another number:");
        let b = read_user_input();
    
        let result = sum(a,b);
        println!("The sum of {} and {} is {}!", a, b, result);
    }
}

fn welcome(first: &String){
    println!("\nHello {}", first);
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}

fn read_user_input() -> u32 {

    let mut input = String::new();
 
    io::stdin().read_line(&mut input).unwrap();
    
    let digit:u32;
    
    match input.trim().parse(){
        Ok(val) => {
            digit = val;
        },
        Err(_err) => {
            println!("This is not a valid input...expected a number.");
            process::exit(1);
        }
    };
    digit
}
