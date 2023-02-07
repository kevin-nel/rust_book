// enums and pattern matching
pub fn chapter_six() {
    // enums can encode meaning along with data
    // they are useful with match expressions
    defining_an_enum();
    match_control_flow();
    if_let_flow_control();

    fn defining_an_enum() {
        // enum lets you say that a value is one of a possible set of values (i.e. Rectangle is one of a set of chapes that could include Circle and Triangle)
        // enums allow for enumeration of their variants
        enum IpAddrKind {
            V4,
            V6,
        }
        // variants of enum are namespaced under its identifier
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        // use in place of a struct
        enum IpAddr {
            V4(String),
            V6(String),
        }
        // data is now attached to each variant of the enum
        // the name of each enum variant now also becomes a function that constructs an instance of the enum.
        // i.e. IpAddr::V4() is a func that takes a string and returns an instance of the IpAddr type
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));

        // each variant can have different types *and* different ammounts of associated data.
        // these types can also be structs
        enum IpAddr2 {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let home = IpAddr2::V4(127, 0, 0, 1);
        let loopback = IpAddr2::V6(String::from("::1"));

        // more complex enum
        enum Message {
            Quit,                    // no data associated
            Move { x: i32, y: i32 }, // named fields like a struct
            Write(String),           // contains a string
            ChangeColor(i32),        // contains an i32
        }
        // equivalent structs (more difficult to assign methods to take any of these methods)
        struct QuitMessage; // unit struct
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // tuple struct
        struct ChangeColorMessage(i32, i32, i32); // tuple struct

        // methods attached to enums
        impl Message {
            fn call(&self) {
                // do function stuff here
            }
        }
        let m = Message::Write(String::from("hello"));
        m.call();

        // Option  Enum (kind of like Null) represents a value being present or not
        /* std lib implementation (even in the prelude)
                enum Option<T>{
                    None,
                    Some(T),
                }
        // <T> is a generic type parameter (i.e.) the Some variant of the Option enum can hol one piece of data of *any* type
        // this makes Option<T> better than Null since it allows Null to have a type and thus never be used in the incorrect situation as if it were a valid value.
        // and there is only a possibility of a null when explicitely using the Option<T> type.
        /*
        // this will not compile
        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        let sum = x + y;
        */
        // can use Some(T) and None without Option::
        */
        // examples
        let some_number = Some(5);
        let some_char = Some('e');
        // requires type annotation because an init value was not given
        let absent_number: Option<i32> = None;
    }
    fn match_control_flow() {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        let floor_coin: Coin = Coin::Quarter(UsState::Alaska);
        value_in_cents(floor_coin);

        fn value_in_cents(coin: Coin) -> i8 {
            match coin {
                // execution stops when it finds a pattern match
                Coin::Penny => {
                    println!("lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    // if quarter has the state variable.
                    // we can now use the state variable in the code assotiated with this arm of the match statement
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
            // difference between match and if:
            // with an if, the condition must evaluate to a boolean. here it evaluates types. there may be other data/parameters associated wiht the type as well that would make a boolean evalutation complicated.
        }
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        // matches must be exhaustive!
        // the arms' patterns must cover all possibilities. (the compiler will warn about this)
        // catch alls and _
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            // catch all, must be at end
            other => move_player(other),
            // _ can be used when whe don't want to use the value in the catch all pattern
            // _ => reroll(),
            // returning an empty tuple means nothing happens unless you roll a 3 or a 7
            // _ => (),
        }
        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {}
    }

    fn if_let_flow_control() {
        let config_max = Some(3u8);
        // using match
        match config_max {
            Some(max) => println!("max is configured as {}", max),
            _ => (),
        }
        // using if let (no exhaustive checking but more concise)
        let mut count = 0;
        if let Some(max) = config_max {
            println!("max is configured as {}", max);
        } else {
            // not required but this will provide the same functionality as _ => () in the match statement
            count += 1
        }
    }
}
