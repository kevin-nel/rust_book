//  let, match, libraries, error handling, static typing and type conversion with shadowing, loop

// bring in the input/output library into scope
// some functions from std are always brought in (called the prelude)
use std::{cmp::Ordering, io};

// need to add rand to Cargo.toml and run cargo install or cargo build
// Cargo lock ensures reproducible build by storing specific versions used in the project
// to use a newer version than the one that was brought in the first time cargo build was run, run cargo update. if you would like to use a new major version then Cargo.toml needs to be updates
// crates.io is a site where you can browse and find crates.
use rand::Rng;

// new public function (accessible from main.rs)
pub fn chapter_two() {
    // guessing game
    // macro of the fmt function that prints to screen
    println!("Guess the number!");

    // thread_rng is the specific generator provided by rand
    // .gen_range takes in range to provide 1..100 is excluesive of 100, 1..=100 is inclusive of upper and lower bounds
    // run cargo doc --open to open documentation for all crates in your project
    // u32 is infered from range
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop is infinite until broken or panic
    loop {
        println!("Please input your guess.");
        // all vars in rust are immutable unless declared with mut as mutable
        // type can often be inferred but needs to be known at compile time (i.e. static typing)
        // String is growable UTF-8 encoded text
        // &str is fixed size
        // ::new() indicates that new is a function associated with the String type
        let mut guess = String::new();
        // user input
        io::stdin()
            // calls read_line method on stdin, &mut guess tells function to store input inside guess (must be mutable)
            // read_line appends to string and does not overwrite contents
            // & indicate a reference which allows multiple parts of the code to access one
            //   piece of data without needing to copy the data into memory multiple times.
            // references are also immutable by default (&guess is immutable and &mut guess is mutable)
            .read_line(&mut guess)
            // handle faulures
            // functions return a Result type with the variants of Ok and Err,
            // Result type has .expect method that can be called. if that instance of Result
            //   is Err .expect() will cause the program to crash and display the error message
            // If Result is an Ok value, Ok will hold the value and .expect will return that value to be used
            // without .expect compiler will error but still compile
            .expect("Failed to read line");

        // using println!
        /*
        let x = 5;
        let y = 10;
        println!("x = {x}, and y + 2 = {}", y + 2);
         */
        println!("You guessed: {guess}");

        // rust allows to shadow the previous value of a variable with a new one.
        // lets use reuse the variable name as a different type instead of creating a new variable name like guess_str and guess_int
        // .trim() removes any leading and trailing whitespace (which readline will have since user needs to press return (\r\n on windows or \n on unix))
        // .parse() converts string to another type this is determined by the type of the variable that the string is parsed into
        // let guess: u32 = guess.trim().parse().expect("Please type a number");

        // improved error handlin
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // loop will not break or panic for type conversion error
            Err(_) => continue,
        };

        // match is made of *arms* wit a pattern to match and code to run is match fits arm
        // .cmp returns a variant of the Ordering enum whicha
        // .cmp assumes both are same type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                // exit loop once won
                break;
            }
        }
    }
}
