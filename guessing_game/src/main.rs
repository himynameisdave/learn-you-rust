//  Import the lib for random numbers (not in the standard libs)
extern crate rand;

//  Gotta do this so that these methods become "traits" of the scope (?)
use std::io;
use std::cmp::Ordering;
use rand::Rng;

//  Programs always start with main()
fn main() {
    //  Initial print & user insult
    println!("Guess the number stupid!");
    //  Generates a random number
    //  gen_range params are lower-inclusive & upper-exclusive
    let secret_number = rand::thread_rng().gen_range(1, 1001);

    // println!("The secret number is: {}", secret_number);

     loop {
        //  mut means this new string is mutable, otherwise immutable by default mafucker
        let mut guess = String::new();
        //  &mut guess is like "hey result of read_line() set guess = result"
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");
        //  wow breaking up method chain, such FP
        //  The guess was a string, but we need to convert it to a Number
        //  (Number aka: u32 aka: unsigned 32 bit integer)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // ignore that it's a string
        };

         match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            },
         }
     }

}
