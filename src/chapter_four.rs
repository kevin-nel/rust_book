// ownership https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
/*

stack vs heap.
ownership is there to help manage heap data.

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

references
- you can have *either* one mutable ref or *any number* of immutable refs
- refs must always be valid (no danglies)

*/
pub fn chapter_four() {
    ownership();
    references();
    slices();
}

fn ownership() {
    vars_and_ownership();
    functions_and_ownership();
    return_vals_and_ownership();

    fn vars_and_ownership() {
        // scoped to the function
        let s = "hello";
        // string literal is stored on the stack since contents are known at compile time

        // scoped to the block
        {
            // t not valid (not yet defined)
            // t valid
            let t = "world";
            // use t for stuff
        } // t moves out of scope and is no longer valid

        let mut s = String::from("hello");
        // Strings are heap allocated and memory must be requested at runtime.
        // ::from() requests memory
        // memory is freed when s goes out of scope
        s.push_str(", world!");
        println!("{}", s);

        // multiple variables and moves
        // bind 5 to x
        let x = 5;
        // bind a copy of x to y
        let y = x;
        // for simple values like ints and string literals this results in 2 vars x and y wiht the same values. this is a simple stack copy.

        let s1 = String::from("hello");
        let s2 = s1;
        // a String is made up of 3 parts, a ptr to the place in memory where the String is stored, the len of the String and the capacity.
        // copying s1 into s2 *does not* copy the actual value in memory (deep copy), it only copies the ptr, len and capacity. (kind of a shallow copy)
        // therefore this is not valid
        // println!("s1 = {}", s1);
        // this results in 2 variables referencing the same area in memory 🚫
        // rust fixes this by making s1 leave scope when s2 is declared (if this was not the case s1 and s2 would both try to free the same area in memory when they leave scope, causing issues) this is called a move
        // rust will never make a deep copy of your data unless explicitely told to, this is good because deep copies are memory intensive and it is good to know when you are doing so and not have it be done automatically

        let s1 = String::from("hello");
        // this *does* copy the heap data as well
        let s2 = s1.clone();
        // this is now valid
        println!("s1 = {}, s2 = {}", s1, s2);

        // types that annotate the *Copy* trait are stored on the stack and are thus copied/cloned and not moved. types with the Drop trait cannot annotate a  Copy trait
        // eg. all ints, bools, floats, char, tuples that only contain types that implement Copy
    }
    fn functions_and_ownership() {
        // s comes into scope
        let s = String::from("hello");
        // s's value moves into the function
        takes_ownership(s);
        // s is no longer valid here

        // x comes into scope
        let x = 5;
        // x would move into function but i32 is Copy, therefore
        makes_copy(x);
        // x is still valid here
    }
    // x and s go out of scope, but s was moved so nothing happens here

    fn takes_ownership(some_string: String) {
        // some string comes into scope
        println!("{}", some_string);
    }
    // some_string goes out of scope and `drop` is called the memory is then freed.

    fn makes_copy(some_int: i32) {
        // some_int comes into scope
        println!("{}", some_int);
    }
    // some_int goes out of scope
    //

    fn return_vals_and_ownership() {
        //gives_ownership moves its return value into s1
        let s1 = gives_ownership();
        // s2 comes into scope
        let s2 = String::from("hello");
        // s2 is moved into takes_and_gives_back, which moves it's return value into s3
        let s3 = takes_and_gives_back(s2);
    }
    // s3 goes out of scope and is dropped. s2 was moved so nothing happens. s1 goes out of scope and is dropped.

    fn gives_ownership() -> String {
        // gives_ownership will move its return value into the function that calls it
        // some_string comes into scope
        let some_string = String::from("yours");

        // some_string is returned and moves out to the calling function
        some_string
    }

    // takes a string and returns one
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope
        // a_string is returned and moves out to the calling function
        a_string
    }

    // assinging a value to another variable moves it. when a variable with heap data goes out of scope, drop will free the memory unless ownership of data has been moved to another variable
}
fn references() {
    // a less tedious way than manually returning and moving ownership is references
    // & allows to refer to a value without taking ownership
    // * is dereferencing but will be discussed later.
    basic_references();
    mutable_references();
    fn basic_references() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("len of '{}', is {}.", s1, len);
    }
    fn calculate_length(s: &String) -> usize {
        // s is a reference to a String (&String), (this is why a string literal is &str)
        s.len()
    }
    // s goes out of scope,  but since calculate_length does not have ownership, it is not dropped

    fn mutable_references() {
        let mut s = String::from("hello");
        change(&mut s);
        // nb! if a mutable ref to a value exists, you can have no other refs to that value.
        /* invalid! cannot borrow `s` as mutable more than once at a time
        let r1 = &mut s;
        let r2 = &mut s;
        println!("{}{}",r1,r2);
        */
        // *can* have multiple mutable references in different scopes
        {
            let r1 = &mut s;
            println!("{}", r1);
        }
        let r2 = &mut s;
        println!("{}", r2);

        // *can* have mutiple immutable references but not if a mutable reference exists
        // immutable references do not expect value to change suddenly.so cannot exist at same time of mutable ref
        /*
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = &mut s; //BIG problem
        println!("{}{}{}", r1, r2, r3);
        */

        // a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        // r1 and r2 are not used beyond here so the references are now out of scope making the next line okay
        let r3 = &mut s;
        println!("{}", r3);
        // if r1 and r2 were used here there would be an issue. (luckily always caught at compile time 🦀)
    }
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // rust does not allow dangling pointers/references (ptr references a location in memry that has been given to someone else)
    // compiler guarantees that res will not go out of scope befrore the reference to the data does.
    /*
     fn dangle() -> &String{ // returns ref to a String
        let s = String::from("hello"); // s is a new String
        &s // return reference to String, s
     } // s goes out of scope, so what is &s now referencing?, this will not compile
    */
    // this works! because of ownership being moved out of function.
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }
    // lifetimes will be discussed in chpt 10
}

fn slices() {
    string_slices();
    other_slices();
    fn string_slices() {
        // long way that returns the index of first space or length if the input is a single word
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }
            s.len()
        }

        let mut s = String::from("hello world");
        let word = first_word(&s);
        // s = ""
        s.clear();
        // word still holds index to the first space of the *previous value* of s. word is now useless. this is not very rusty :(

        // string slices!!
        let s = String::from("hello world");
        // hello and world are references to a portion of String s
        let hello = &s[0..5];
        let world = &s[6..11];
        // range syntax
        // [..2] = [0..2]
        // [2..s.len()] = [2..]
        // [0..s.len()] = [..]

        fn better_first_word(s: &String) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
            &s[..]
        }
        let mut s = String::from("hello world");
        let word = better_first_word(&s);
        // s = ""
        // s.clear();
        // trying to clear s will result in compile error since a reference to it is borrowed as immutable by the function so cannot be mutated.
        println!("the first word is: {}", word);

        // STRING LITERALS

        // string literals are slices of a specific portion of the stack in the compiled binary
        let s: &str = "hello world";

        // using &str allows the function to work for literals too,and slices of a String (whole or partial)
        fn even_better_first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
            &s[..]
        }
        let mut s = String::from("hello world");
        let word_s = even_better_first_word(&s[..]);
        let sl: &str = "my string literal";
        let word_sl = even_better_first_word(sl);
    }

    fn other_slices() {
        let a = [1, 2, 3, 4, 5];
        // slices store a reference to the first element and the length and work for many types of collections
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
}
