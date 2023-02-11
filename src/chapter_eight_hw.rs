// notes:
// what is best practice for passing strings, vectors, arrays and hashmaps to an returning from functions?
// what is happening to these values with regards to memory and ownership and they move in and out of functions and scope?

pub fn chapter_eight_hw() {
    median_mode(&[0, 1, 3, 4, 5, 5, 5, 6, 9, 2, 4, 9]);
    string_to_pig_latin(&String::from("happy birth day my dude! 😋"));
    add_employee();
}

struct Stats {
    median: i32,
    mode: i32,
}
fn median_mode(arr: &[i32]) -> Stats {
    // get median and mode of list of numbers using a hash map
    let stats = Stats { median: 0, mode: 0 };
    stats
}
fn string_to_pig_latin(sentence: &str) -> String {
    String::from("not yet implemented")
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
}

fn add_employee() {
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
}
