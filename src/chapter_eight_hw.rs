// notes:
// what is best practice for passing strings, vectors, arrays and hashmaps to an returning from functions?
// what is happening to these values with regards to memory and ownership and they move in and out of functions and scope?

use std::{
    any,
    collections::{btree_map::Values, HashMap},
    io,
    ops::Index,
};

use rand::Error;

pub fn chapter_eight_hw() {
    // median and mode of Vec<i32>
    let mut data = vec![0, 1, 3, 4, 5, 5, 5, 6, 9, 2, 4, 9];
    let data = Data::new(&mut data); // the vector is now sorted and stored in a Data struct
    println!("{:?}", data);

    // this gives an error
    let ig_pay_atin_lay = string_to_pig_latin(&String::from("happy birth day my dude! 😋"));
    println!("{ig_pay_atin_lay}");
    // this works
    let ig_pay_atin_lay = string_to_pig_latin(&String::from("happy birth day my dude!"));
    println!("{ig_pay_atin_lay}");

    add_employee();
}

// get median and mode of vector of integers by returning a data structure with the sorted vector
// along with the median and mode
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

// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
fn string_to_pig_latin(sentence: &String) -> String {
    let vowels = "aeiou";
    let mut pig_latin = String::from("");
    for slice in sentence.split_whitespace() {
        if slice.is_ascii() == true {
            if vowels.contains(slice.chars().nth(0).unwrap()) == false {
                let first_word = slice.get(1..).unwrap();
                let second_word = slice.chars().nth(0).unwrap();
                pig_latin = format!("{} {}-{}ay", pig_latin, first_word, second_word);
            } else {
                pig_latin = format!("{} {}-hay", pig_latin, slice);
            }
        } else {
            return format!("invalid string: not ascii\ninput string: {}", sentence);
        }
    }

    pig_latin = pig_latin[1..].to_string();
    pig_latin
}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

// I must admit that I was mostly debugging using the compiler errors and not thinking too much
// about where things are stored in memory, I also don't understand why you would use a hashmap
// for this since they are not particularly good at sorting and i needed to make a vector to do
// that anyway.
// i tried to do a lot of error catching and make sure that all names and departments were
// capitalised correctly to avoid duplicate entries

// i did not consider the cause of names with more than one word though that could most likely be
// solved by adding another arm to the length match statement or splitting the department at the
// intermediate word ("to") instead of relying on argument indices
fn add_employee() {
    let mut department = String::new();
    let mut employee_name = String::new();
    let mut employees: HashMap<String, String> = HashMap::new();

    println!("Add employees and their departments.\ne.g. \"add Sally to Engineering\"\nTo view all employees and their departments type \"list all\"\nTo view all employess in a specific department type \"list department\", where \"department\" is the name of the department.\nTo exit type\"exit\"\n");

    loop {
        // clear input so that io::stdin().read_line(&mut input) doest append the next input
        let mut input = String::from("");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        // let input = input.to_lowercase();

        let arguments = input.split_whitespace().collect::<Vec<&str>>();
        // probably a rustier way of doing this error checking
        match arguments.len() {
            0 => continue,
            1 => match &arguments[0][..] {
                "exit" => break,
                _ => {
                    println!("expected multiple parameters or \"exit\"");
                    continue;
                }
            },
            2 | 3 | 4 => {}
            _ => {
                println!("expected multiple parameters");
                break;
            }
        }

        if arguments.len() < 2 {
            println!("expected multiple parameters");
            continue;
        }

        match arguments[0].to_lowercase().as_str() {
            // this is probably unnecessary since it was covered earlier
            "exit" => break,
            "add" => {
                // capitalise first letter of words
                employee_name = format!(
                    "{}{}",
                    arguments[1]
                        .chars()
                        .nth(0)
                        .expect("some kind of unicode error i guess 🤷‍♀️")
                        .to_uppercase(),
                    arguments[1][1..].to_lowercase()
                );

                match arguments.len() {
                    3 => {
                        department = format!(
                            "{}{}",
                            arguments[2]
                                .chars()
                                .nth(0)
                                .expect("some kind of unicode error i guess 🤷‍♀️")
                                .to_uppercase(),
                            arguments[2][1..].to_lowercase()
                        )
                    }
                    4 => {
                        department = format!(
                            "{}{}",
                            arguments[3]
                                .chars()
                                .nth(0)
                                .expect("some kind of unicode error i guess 🤷‍♀️")
                                .to_uppercase(),
                            arguments[3][1..].to_lowercase()
                        )
                    }
                    _ => {
                        department = String::from("");
                        println!("error! expected either \"add name to department\" or \"add name department\".");
                        continue;
                    }
                }
                // insert name and department, if name already exists update department
                employees.entry(employee_name).or_insert(department);
            }
            "list" => match arguments[1] {
                "all" => println!("{:?}", employees),
                _ => {
                    let department = format!(
                        "{}{}",
                        arguments[1]
                            .chars()
                            .nth(0)
                            .expect("some kind of unicode error i guess 🤷‍♀️")
                            .to_uppercase(),
                        arguments[1][1..].to_lowercase()
                    );

                    // if able to find input in the departments within the hashmap print the employes (key) with that department (value)
                    let contains_dept = employees.values().any(|x| x == &department);

                    match contains_dept {
                        // make new vector containing employees with dept as value
                        true => {
                            let mut employees_in_dept = Vec::new();

                            for employee in &employees {
                                if employee.1 == &department {
                                    employees_in_dept.push(employee.0);
                                }
                            }
                            // alphabetical sort
                            employees_in_dept.sort_by(|a, b| a.cmp(b));
                            println!("{:?}", employees_in_dept);
                        }
                        false => {
                            println!("expected either an \"add\" or \"list\" command");
                            continue;
                        }
                    }
                }
            },
            _ => {
                println!("expected either an \"add\" or \"list\" command");
                continue;
            }
        }
    }
}
