// collections
// collections point to a value on the heap (growable)
pub fn chapter_eight() {
    vectors();
    strings();
    hash_maps();
    fn vectors() {
        //  -------- dynamic arrays --------
        // vectors are implemented using generics (Vec<T> can hold any type)
        // empty
        let v: Vec<i32> = Vec::new();
        // init values (type is infered)
        // vec! macro
        let v = vec![1, 2, 3];
        let mut v = Vec::new();
        v.push(5);
        // once a value is pushed a type for the vector can be inferred otherwise a type must be explicitely declared
        v.push(6);
        v.push(7);
        v.push(8);
        v.push(9);

        //  -------- reading --------
        let v = vec![1, 2, 3, 4, 5];

        // reference to the element
        let third: &i32 = &v[2];
        println!("the third element is {third}");

        // get method
        let third: Option<&i32> = v.get(2);
        // checking if element exists since it is dynamic
        match third {
            Some(third) => println!("the third element is {third}"),
            None => println!("there is no third element"),
        }

        let v = vec![1, 2, 3, 4, 5];
        // this will panic 🚫
        /*
        let does_not_exist = &v[100];
        */
        // this will not panic it will return None (which ideally needs to be handled with either Some(&element) or None (chapter 6)
        let does_not_exist = v.get(100);
        // remember cannot have mutable and immutable references in thesame scope.
        /* this doesnt work
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        v.push(6);
        println!("The first element is: {first}");
        */
        // because adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space so even an immutable reference to an individual element cannot exist at same time as mutuable one to anywhere else in the vector

        //  -------- iterating --------
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }
        // or
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            // need to de reference because &v is just a ptr to the memory
            *i += 50;
        }

        // enums to store mutlipe types
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        // rust needs to know what types the vector will hold at compile time

        // dropping a vector drops its elements
        {
            let v = vec![1, 2, 3, 4];
        } // v goes out of scope and is freed
    }
    fn strings() {
        // strings are collections of bytes
        let mut s = String::new();
        let data = "initial contents";
        let s = data.to_string();
        // also works on literals directly
        let s = "initial contents".to_string();
        // from
        let s = String::from("initial contents");
        // they are utf-8
        let hello = String::from("안녕하세요");

        //  -------- updating --------
        let mut s = String::from("foo");
        s.push_str("bar");
        // s = "foobar"
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        // push_str takes a string slice and does not take ownership so s2 is still valid after use
        println!("s2 is {s2}");
        // just add one character
        s1.push('t');

        //  -------- concatenation --------
        let s1 = String::from("hello, ");
        let s2 = String::from("world!");
        // cannot add two Strings only a String and a &str, the compiler coerces &String into &str in this case turning &s2 into &s2[..]
        let s3 = s1 + &s2;
        // s1 was moved and can no longer be used
        // s2 is still valid str after this
        // this is because + is actuall an add() method
        // fn add(self, s: &str) -> String {
        // takes ownership of self and appends a copy of &str
        // concatenating multiple strings with the plus operator becomes messy
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = s1 + "-" + &s2 + "-" + &s3;
        // use format! instead
        let s1 = String::from("tic"); // s1 is not valid from previous declaration (could also use s1.clone() in the concatenation but that would increase memory usage i guess)
        let s = format!("{s1}-{s2}-{s3}");
        println!("{s}");
        // format doesn't take ownership of s1 so all are usable afterwards

        //  -------- indexing --------
        let s1 = String::from("hello");
        /* this doesnt work because Strings are not iterable. they are a wrapper over a Vec<u8>
        let h = s1[0];
        */
        // "hello" used 5 bytes since each char is 1 byte
        // Здравствуйте is 24 bytes since each unicode uses 2 bytes
        // indexing is therefore a little more complicated
        /* also invalid
        let hello = "Здравствуйте";
        let answer = &hello[0];
        */
        /* another example
        नमस्ते is one 'word'

        in bytes:
        [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

        as Unicode scalar values:
        ['न', 'म', 'स', '्', 'त', 'े']

        grapheme clusters or 'words'
        ["न", "म", "स्", "ते"]
        */
        // to index we need to slice
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        // not every unicode is 4 bytes so you will have to know in advance using a range that does not end at a character boundary will panic the program
        println!("{s}");
        // methods for iteration
        // chars and bytes are different
        for c in s.chars() {
            println!("{c}") // З and д
        }
        for b in s.bytes() {
            println!("{b}") // 208 151 208 180
        }
        // getting grapheme clusters is complex so is not in the std library
        // other cool methods (contain, replace)
    }

    fn hash_maps() {
        // store key value pairs
        use std::collections::HashMap;
        let mut scores = HashMap::new();
        // adding
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("yellow"), 50);
        // accessing
        let team_name = String::from("blue");
        // get() returns Option<&V> if there is no value it will return None
        // copied gets an Option<i32> instead of the Option<&i32> returned by get()
        // unwrap_or() sets score to 0 if scores doesn't have an entry
        // this line therefore handles cases where no value is paired to the key
        let score = scores.get(&team_name).copied().unwrap_or(0);

        //  -------- iterate --------
        for (key, value) in &scores {
            // prints the pair in an arbitrary order
            println!("{key}: {value}");
        }

        //  -------- ownership --------
        // Copy trait types like i32 are copied to the hash map, for owned values like String they will be moved and the hasm map will be the owner of those values
        let field_name = String::from("favorite color");
        let field_value = String::from("blue");
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // cannot use field_name and field_value here since they were moved

        //  -------- updating --------
        // overwrite
        let mut scores = HashMap::new();
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("blue"), 25);
        println!("{:?}", scores);

        //adding only if not present
        scores.entry(String::from("yellow")).or_insert(50); // inserts as new value because it doesn't exist
        scores.entry(String::from("blue")).or_insert(50); // changes value in existing key-value pair
        println!("{:?}", scores);

        // updating based on old value
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        // split_whitespace method returns an iterator over sub-slices, separated by whitespace,
        for word in text.split_whitespace() {
            // or_insert method returns a mutable reference (&mut V) to the value for the specified key
            let count: &mut i32 = map.entry(word).or_insert(0);
            *count += 1;
            /*
            // Rust          C/C++
                a: &T     == const T* const a; // can't mutate either
            mut a: &T     == const T* a;       // can't mutate what is pointed to
                a: &mut T == T* const a;       // can't mutate pointer
            mut a: &mut T == T* a;             // can mutate both
            */
            // this is quite confusing tbh. hopefully the compiler helps out when there is an issue
            // 😵‍💫
            /* heres another table that can help
            let x = value;
              x {binds immutably} to {immutable value}

            let mut x = value;
              x {binds mutably} to {possibly mutable value}

            let x = &value;
              x {binds immutably} to {a reference to} {immutable value}

            let x = &mut value;
              x {binds immutably} to {a reference to} {mutable value}
              // cannot change the reference but can change the value?

            let mut x = &value;
              x {binds mutably} to {a reference to} {immutable value}

            let mut x = &mut value;
              x {binds mutably} to {a reference to} {mutable value}
            */
            // both from https://stackoverflow.com/questions/28587698/whats-the-difference-between-placing-mut-before-a-variable-name-and-after-the
        }

        // {"world": 2, "hello": 1, "wonderful": 1}
        println!("{:?}", map);

        //  -------- hashing functions --------
        // By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables
        // you can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait
    }
}
