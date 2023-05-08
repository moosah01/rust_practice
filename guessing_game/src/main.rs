use ::rand::Rng; //use brings Rng trait into scope
use ::std::io; //use brings io library into scope
use std::cmp::Ordering; //use brings Ordering enum into scope
use colored::*; //use brings colored library into scope

fn main() {
    println!("Hello Students! Welcome to Moosa's guess the number game!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101); //gen_range is an inclusive range
    println!("The secret number is {}", secret_number);

    loop {
        let mut guess = String::new(); //new returns an empty string

        io::stdin()
            .read_line(&mut guess) //read_line takes whatever the user types into standard input and places that into a string
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        //trim removes whitespace and parse parses a string into a number

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break; //breaks out of the loop
            }
        }
    }
}
