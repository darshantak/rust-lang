use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    
        
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is {}",secret_number);
    loop{
        let mut guess = String::new();
        println!("Welcome to the guessing game");
        println!("Please input a number");
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {} ",guess);
    let guess:u32=guess.trim().parse()
    .expect("Failed to type convert");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {println!("You win");
    break;},
    }
    }
    
    
}
