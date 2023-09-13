use std::io; //io library
use rand::Rng; //crate
use std::cmp::Ordering; //match crate

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //random number generator

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //stocke la string dans une variable

        io::stdin() //prends l'user input
            .read_line(&mut guess) //appelle la read line method
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){ //converti string en 32 bit nb avec parse
            Ok(num) => num,
            Err(_) => continue,
        };
    

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }
    }
}