use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
    net::IpAddr,
};

pub fn chapter_nine() {
    // recoverable (report and retry): Result<T,E>
    recoverable();
    // unrecoverable (panic): panic!()
    unrecoverable();

    /*
    adding
    [profile.release]
    panic = 'abort'
    to Cargo.toml will prevent unwinding (tracing back through the stack and cleaning up data from each function)
    */
    to_panic_or_not_to_panic();
}
fn unrecoverable() {
    //panic!("at the disco");

    //let v = vec![1, 2, 3];
    //v[99];

    // backtrace is a list of all the functions that have been called to get to this point

    // RUST_BACKTRACE=1 cargo run

    // to exclude debug symbols compile with --release
}

fn recoverable() {
    /*
    enum Result<T,E>{
        Ok(T),
        Err(E),
    }
    */
    // T and E are generic types, T will be returne for Ok variants of the Result enum and E will b returned for the Err variant
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        // returns the inner file out of the Result
        Ok(file) => file,
        // check what kind of error it was
        Err(error) => match error.kind() {
            // if file not found, create new file (NotFound is a variant of ErrorKind)
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                // panic if couldn't create the file
                Err(e) => panic!("problem creating the file: {:?}", e),
            },
            other_error => {
                // panic if theres some other error opening the file but it does exist
                panic!("problem opening file: {:?}", error);
            }
        },
    };

    // we can also use methods with closures (i lowkey think this is cursed as m8 ☠️, this is what I think java looks like. *i have never used java*)
    let greeting_file_result = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("problem creating the file {:?}", error);
            })
        } else {
            panic!("problem opening the file {:?}", error);
        }
    });

    // shorcuts (using unwrap and expect to panic on erri)
    // errors or returns file handle
    let greeting_file_result = File::open("hello.txt").unwrap();
    // errors with specified msg or returns file handle
    let greeting_file_result =
        File::open("hello.txt").expect("hello.txt should be included in the porject");

    // error propagation
    // functions that call something that might file can return error to calling code
    // this part of the code is now responsible for handling the errors from read_username_from_file()

    read_username_from_file();
    read_username_from_file_propagation_operator();
    last_char_of_first_line_propagation_operator("some string slice\nwith\nmutliple\nlines");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        // return early from function with error from file opening issue
        Err(e) => return Err(e),
    };
    let mut username = String::new();

    // returns Result type
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        // return error from read to string
        Err(e) => Err(e),
    }
    // ^ no semicolon here so this function returns the Result type containing info about errors
    // this function has 2 operations that could erro and returns Result type containing these errors
}

fn read_username_from_file_propagation_operator() -> Result<String, io::Error> {
    // the ? operator behaves the similarly to the match stateents above
    // unlike the match statements error values with the ? called on them go through the `from` function (used to convert values from one type inot another). this converts the error type into the error type defined by the return type of the function (io::Error in this case)
    // we could define a custom error type OurError and impl From<io::Error> for OurError type to construct an instance of OurError from an io::Error

    /*
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
    */

    // or shorter

    /*
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
    */

    // even shorterer
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line_propagation_operator(text: &str) -> Option<char> {
    // The ? operator can only be used in functions whose return type is compatible with the value the ? is used
    // it also works with the Option type
    text.lines().next()?.chars().last()

    // ? can work on Result in fn that returns Result OR Option in fn that returns Option but not mix/match
    // ok method on Result or the ok_or method on Option to do the conversion explicitly.
}

// main() can also return Result<(), E> this lets us use the ? operator without needing other functions

/* for eg.
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
*/
// Box<dyn Error> is a trait object but can be read as 'any type of error'
// exe will exit with 0 if main returns Ok(()) and exit with a non zero value if it returns an Err
//  the std::process::Termination trait contain a report fn that returns an ExitCode

fn to_panic_or_not_to_panic() {
    // , it’s understood that a call to a method like unwrap that could panic is meant as a placeholder for the way you’d want your application to handle errors,
    //unwrap and expect methods are very handy when prototyping, before you’re ready to decide how to handle errors.
    // in tests you want the whole test to fail

    // call unwrap or expect when you have some other logic that ensure the Result will have an Ok value. (ie when you know more than the compiler)
    let home: IpAddr = "127.0.0.1".parse().expect("hardcoded IP should be valid");

    // It’s advisable to have your code panic when it’s possible that your code could end up in a bad state.
    /*
    - The bad state is something that is unexpected, (not something that might happen occassionaly, like a user entering data in the wrong format.)
    - Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
    - There’s not a good way to encode this information in the types you use.
    */

    // return errors if you are writing a library, panic if something cuold cause stuff to be insecure or harmful

    // when errors are expected its better to return a result

    // verify that inputs are valid first so that you can panic before using them
    // fn s have 'contracts', outputs are only guaranteed if inputs meet requirements. panicking on contract violation shows a caller side issue

    // try avoid having tons of error checks by using rust's type system
    // if you specify a type then you don't need to handle 2 cases for Some and None for Option.
    // use unsigned when values shouldn't be negative
    // etc

    custom_types_for_validatio();
}

fn custom_types_for_validatio() {
    loop {
        let guess = "12";
        // parse as signed value
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // check that value is positive
        if guess < 1 || guess > 100 {
            println!("the secret number will be between 1 and 100.");
            continue;
        }
        break;
    }
}
// better way
pub struct Guess {
    // use i32 to allow negative inputs
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        // check that input is positive
        if value < 1 || value > 100 {
            // contract violated for Guess::new
            panic!("guess must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
    // borrows self, and returns i32. (getter)
    // needed because the value field of Guess struct is private
    // keeping the value private keeps code from setting value directly without the checking that is provided by new
    pub fn value(&self) -> i32 {
        self.value
    }
}

// in conclusion, rust has loads of ways to handle errors and it *really* wants you to handle them
