// notes:
// what is best practice for passing strings, vectors, arrays and hashmaps to an returning from functions?
// what is happening to these values with regards to memory and ownership and they move in and out of functions and scope?

use std::collections::HashMap;

pub fn chapter_eight_hw() {
    // median and mode of Vec<i32>
    let mut data = vec![0, 1, 3, 4, 5, 5, 5, 6, 9, 2, 4, 9];
    let data = Data::new(&mut data); // the vector is now sorted and stored in a Data struct
    println!("{:?}", data);

    string_to_pig_latin(&String::from("happy birth day my dude! 😋"));
    add_employee();
}

#[derive(Debug)]
struct Data {
    data: Vec<i32>,
    median: i32,
    mode: i32,
}
impl Data {
    fn new(data_vec: &mut Vec<i32>) -> Self {
        let mut map = HashMap::new();
        let mut data_mode = 0;
        let mut data_median = 0;

        // iterates through array
        for element in data_vec.into_iter() {
            // adds elements to array with a count, increments the count if element already exists
            let count: &mut i32 = map.entry(element).or_insert(0);
            *count += 1;
        }

        // find value with highest frequency (mode)
        let mut count = 0;
        for (key, val) in map.iter() {
            if val > &count {
                data_mode = **key;
                count += 1;
            }
        }

        data_vec.sort();
        let len = data_vec.len();
        if len % 2 != 0 {
            let data_median = ((data_vec[len / 2 - 1] + data_vec[len / 2 + 1]) as f32) / 2.0;
        } else {
            data_median = data_vec[len / 2];
        }

        Self {
            data: data_vec.to_vec(),
            median: data_median,
            mode: data_mode,
        }
    }
}

fn string_to_pig_latin(sentence: &str) -> String {
    String::from("not yet implemented")
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
}

fn add_employee() {
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
}
