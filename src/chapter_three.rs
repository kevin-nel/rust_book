use std::{num, result};

// common programming concepts
// comments should be placed on the line above line being commented on
pub fn chapter_three() {
    variables();
    constants();
    shadowing();
    scalar_data_types();
    operators();
    compound_data_types();
    functions();
    control_flow();
}

fn variables() {
    // use snake case this_is_a_var
    // variables and mutability
    let x = 5;
    println!("x is {}", x);
    // x = 6; will panic

    let mut y = 5;
    println!("y is {}", y);
    y = 6;
    println!("y is {}", y);
}

fn constants() {
    // use all caps
    // arent allowed to use mut
    // type MUST be allocated (no inference)
    // 60 * 60 * 3 is evaluated at compile time (for let statements this is not always the case)
    // constants are valid for entire time the programs runs,
    // within the scope they were declared
    // rust is block scoped so this const will only be valid within this function
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}

fn shadowing() {
    let x = 5;
    // create new variable x taking original value.
    let x = x + 1;
    {
        let x = x * 2;
        println!("x in the inner scope: {}", x); // 12
    }
    println!("x in the outer scope: {}", x); // 6

    // different from mutability
    // we can reuse the variable name with different types.
    let spaces = "    ";
    // unused variables should be prefixed by _
    let _spaces = spaces.len();

    // the following doesn't compile
    // not allowed to mutate a variable’s type
    /*
        let mut spaces = "    ";
        spaces = spaces.len();
    */
}

fn scalar_data_types() {
    // rust is statically typed (must know type of all variables at compile time)
    // rust can often infer but when multiple types are possible the type must be annotated like below (could be i32, u8 etc)
    let _guess: u32 = "42".parse().expect("Not a number!");

    // scalar types (a single value)
    ints();
    floats();
    bools();
    chars();

    fn ints() {
        /*
        // integers
        - signed => possible to be negative (stored as 2s complement
            - can store from - 2^(n - 1) to 2^(n - 1) - 1, inclusive
            - eg i8 : -128 to 127
        - unsigned => only positive
            - can store from 0 to 2^(n) - 1
            - eg u8 : 0 to 255
        - unsigned => only positive
        - integer literals
            - decimal : 98_222 (_ is a visual indicator to make large numbers easier to real)
            - hex : 0xff
            - octal : 0o77
            - binary : 0b1111_0000
            - byte (only u8) : b'A'
        - when numbers are used without declaration you can add a type suffix: 57u8
        - * compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Rust used 2s complement wrapping, for a u8 256 becomes 0 and 257 becomes 1. Relying on integer overflow is an error*
            - explicitely handle wrapping
                - wrapping_* methods
                - return None if there is overflow using the checked_* methods
                - return the value and a bool indicating there was overflow with the overflowing_* methods
                - saturate at max value with saturating_* methods
        */

        let a: u8 = 0xfe; // 254
        println!("{}", a + 0x1u8); // 255
    }

    fn floats() {
        /*
         - floats (always signed)
            - use the IEEE-754 std
            - f32 is single precision
            - f64 is double precision (default type)
        */
        let _x = 2.0; // f64
        let _y: f32 = 3.0; // f34
    }
    fn bools() {
        // one byte in size
        // mainly used in if statements
        let _t = true;
        let _f: bool = false;
    }
    fn chars() {
        // like c, single quote for char double for str
        // chars are unicode
        let c = 'z';
        let z: char = '↯';
        let heart_cat = '😻';
        println!("here are some chars {0}{1}{2}", c, z, heart_cat);
    }
}

fn operators() {
    let sum = 5 + 10;
    println!("sum of 5 and 10: {}", sum);
    let diff = 95.5 - 4.3;
    println!("difference of 95.5 and 4.3: {}", diff);
    let prod = 4 * 30;
    println!("product of 4 and 30: {}", prod);
    let quotient = 56.7 / 32.2;
    println!("quotient of 56.7 and 32.2: {}", quotient);
    let trunc = -5 / 10; //-1
    println!("truncation of -5 and 10 (-5/10): {}", trunc);
    let remainder = 43 % 10; //3
    println!("remander (modulus) of 43 and 10: {}", remainder);
}

fn compound_data_types() {
    tuples();
    arrays();

    fn tuples() {
        // group togeteher multiple values with variety of types
        let tup = (500, 6.4, 1);
        // takes the tuple and turns it into 3 separate variables
        let (_x, _y, _z) = tup;
        // access tuple elements with .

        let x: (i32, f64, u8, char) = (500, 6.4, 1, 'B');
        let _five_hundred = x.0;
        let _six_point_four = x.1;
        let _one = x.2;
        let _b = x.3;

        // an empty tuple is called a unit
        // expressions (functions) return this if they dont return any other value
        let unit = ();
    }
    fn arrays() {
        // allocated on stack, fixed size (not growable)
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        let b = [3; 5]; // [3,3,3,3,3]
                        // access
        let first = a[0];
        let second = a[1];

        // will panic at runtime when i = 6
        /*
         for i in 0..=6 {
            println!("accessing the {}th element of a: {}", i, a[i]);
        }
        */
    }
}

fn functions() {
    parameters(15, 'm');
    let x = returns();
    println!("x: {}", x);

    // should be defined after main but it doesnt matter so long as functions are withing scope of call
    fn parameters(x: i32, unit_label: char) {
        // types MUST be declared (no inference)
        println!("measurement: {}{}", x, unit_label);
    }

    // statements: instrusctions performing an action without returning a value
    // statement has no return ∴ let x = (let y = 6); is not valid (neither is x=y=6)
    let y = 6;

    // expressions: evaluate to a resultant value (and return)
    // returns have no semicolon (semicolon will get rid of return value)
    let y = {
        let x = 3;
        x + 1
    };
    println!("y: {}", y);

    // functions with returns
    // must define type of return value
    fn returns() -> i32 {
        fn plus_one(x: i32) -> i32 {
            x + 1
        }
        // no semicolon so that the functions returns() will return 5
        // function will return (), an empty tuple, when no return value (or line ends in semicolon)
        plus_one(4)
    }
}

fn control_flow() {
    if_statements();
    loops();

    fn if_statements() {
        let number = 3;
        // conditions in if statements must be bool (ie cannot do if number {})
        if number < 5 {
            println!("true");
            println!("number is less than 5");
        } else {
            println!("false");
            println!("number is greater than 5");
        }

        if number != 0 {
            println!("true");
            println!("number is not equal to zero");
        }

        // rust only executes up to the first 'true'
        // match is better and more clear in most cases
        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 4");
        } else if number % 2 == 0 {
            println!("number is divisible by 4");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }

        // ifs in lets
        // an if is an expression so 5 and 6 are returned when condition is either true or false
        let condition = true;
        let number = if condition { 5 } else { 6 };
        // let number = if condition { 5 } else { "six" }; // cannot do this vars must have a single type at compile time (statically typed)

        // blocks of code evaluate to last expression in them and numbers are also expressions
    }
    fn loops() {
        // three kinds of loops loop, while, for
        loop_loop();
        while_loop();
        for_loop();
        fn loop_loop() {
            let mut x: u8 = 0;
            loop {
                // loop is infinite until broken
                x += 1;

                if x > 5 {
                    break;
                } else if x == 3 {
                    continue; // skips any remaining code in loop and loops again
                              // this loop will print 1,2,4,5 skipping 3
                }
                println!("x : {}", x);
            }
            // return from loop (add value you want returned after break used to stop loop)
            let mut counter = 0;
            let result = loop {
                counter += 1;
                if counter == 10 {
                    break counter * 2;
                }
            };

            println!("The result is {result}");

            // nested loops and loop labels
            // break and continue apply to innermost loop at that point
            let mut count = 0;
            'counting_up: loop {
                println!("count: {}", count);
                let mut remaining = 10;
                loop {
                    println!("remaining: {}", remaining);
                    if remaining == 9 {
                        break;
                    }
                    if count == 2 {
                        break 'counting_up; // allows breaking of outer loop from within an inner loop
                    }
                    remaining -= 1;
                }
                count += 1;
            }

            println!("end count: {}", count);
        }

        // typically clearer than a loop loop
        fn while_loop() {
            let mut x = 10;
            while x != 0 {
                // calls breaks when condition evaluates to true
                println!("I'm not finished yet!");
                x -= 1;
            }
            // looping through a collection with while
            let a = [10, 20, 30, 40, 50];
            let mut idx = 0;
            while idx < 5 {
                println!("a[{0}] = {1}", idx, a[idx]);
                idx += 1;
            }
        }

        // most used loop, safe and concise.
        fn for_loop() {
            // _ is unused just loops for 5 times
            for _ in 1..6 {
                println!("I'm not finished yet!");
            }

            // in reverse?
            for number in (1..6).rev() {
                println!("{number}");
            }

            // looping through a collection with for
            let a = [10, 20, 30, 40, 50];
            for element in a {
                println!("a = {element}")
            }
        }
    }
}
