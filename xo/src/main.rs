
fn main() {
    let square = "|   ";
    let horiz = " ".to_owned() + &"-".repeat(13);
    let dims = 3;
    // let mut player = 0; //0 is x, 1 is o
    let mut bot = "   ".to_owned();
    // print game board
    
    for n in 1..dims+1 {
        println!("{}", horiz);
        println!("{}{}", n, square.repeat(dims+1));
        bot = bot + &n.to_string() + &"   ".to_owned();
        if n == dims {
            println!("{}", horiz);
            println!("{}", bot)
        }
    }

    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Player 1, please make a move: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    let row_b: u8 = s.as_bytes()[1];
    let col_b: u8 = s.as_bytes()[0];

    let row: char = row_b as char;
    let col: char = col_b as char;
    println!("you chose row {} and col {}", row, col );

}