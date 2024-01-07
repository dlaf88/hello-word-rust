use rand::Rng;
use std::cmp::Ordering;
use std::io;
struct GuessString {
    first_phrase: String,
    second_phrase: String,
    third_phrase: String,
}

fn main() {
    let guess_string = GuessString {
        first_phrase: String::from("Let's play a game!"),
        second_phrase: String::from("Guess the number!"),
        third_phrase: String::from("Please input your guess."),
    };
    println!("{}", guess_string.first_phrase);
    println!("{}", guess_string.second_phrase);
    println!("{}", guess_string.third_phrase);
    let secret_number = rand::thread_rng().gen_range(1..=101);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
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
