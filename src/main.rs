use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please enter your guess");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{


    let mut guess = String::new();
    // The `read_line` method appends data to the string, so we need to clear it first

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read the line");

    println!("Your entered guess is:{}", guess);

     // Now what i'm about to do is called a pro gamer move: Shadowing - by shadowing, a variable can be reused, like here i'm going to convert guess, which is a string, into a number type, u32 more specifically.
    // this feature is often used when you want to convert a value from one type to another type.

// // Parse method returns the Result enum which contains two variants Ok and err
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };

    // trim(): Removes any whitespaces from beginning or end of the string also remove the new line /n character
    // parse(): The parse method on strings converts a string to another type.
    // guess : u32 = annotes the compiler to type conversion


        println!("Please enter the guess: ");

        match guess.cmp(&secret_number){ // Here cmp method returns one of the variants : Greater, less or Equal. the match expression is made up of arms. An arm consists of a pattern to be matched  against. if the pattern matches then the code of the corresponding arm must be run. Ordering matches the arm pattern and contains the code
            Ordering :: Less => println!("Too small"),
            Ordering :: Equal => {
                println!("You won!");
                break;
            },
            Ordering :: Greater => println!("Too large"),
        }
    }
}
