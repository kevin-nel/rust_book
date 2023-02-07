// structs are custom data types kinda like objects
pub fn chapter_five() {
    defining_and_instantiating_structs();
    example_using_structs();
    method_syntax();

    fn defining_and_instantiating_structs() {
        tuple_structs();
        normal_structs();

        fn normal_structs() {
            #[derive(Debug)] // make struct printable with debug trait {:?}
                             // also possible to print a struct using dbg!(&user1); without deriving the debug trait
            struct User {
                active: bool,
                username: String,
                email: String,
                sign_in_count: u64,
            }
            // using a struct after definition by instantiating
            let user1 = User {
                active: true,
                username: String::from("theLegend27"),
                email: String::from("l33tg4m3r@yahoo.com"),
                sign_in_count: 1,
            };
            println!("{:?}", user1);
            dbg!(&user1); // prints to stderr

            // accessing data from struct
            // entire struct must be mutable (cannot make only certain fields mutable )
            let mut user1 = user1;
            // changing value in the email field
            user1.email = String::from("george_costanza@seinfeld.net");
            println!("{:?}", user1);

            fn build_user(email: String, username: String) -> User {
                User {
                    active: true,
                    username, // field init shortand: equivalent to username: username,
                    email,
                    sign_in_count: 1,
                }
            }
            let user2 = build_user(
                "jerry_seinfeld".to_string(),
                "jerry@seinfeild.net".to_string(),
            );
            println!("{:?}", user2);

            //struct update (move data from struct to new struct using values from previous struct)
            let user3 = User {
                active: user1.active, // moves active value of user1 into user3
                username: user1.username,
                email: String::from("art_vandalay@seinfeld.net"),
                sign_in_count: user1.sign_in_count,
            };
            println!("{:?}", user3);
            // String does not have Copy so info from user1 that was moved to user3 is no longer accessible since the memory would be freed as the data goes out of scope
            // struct update syntax
            let user4 = User {
                email: String::from("art_vandalay@seinfeld.net"),
                ..user2 // fill rest of struct with values from user1 (excl email)
            };
            println!("{:?}", user4);
        }
        // TUPLE structs
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        fn tuple_structs() {
            let black = Color(0, 0, 0);
            let origin = Color(0, 0, 0);
        }
        // unit structs with no fields (kind of like how defines are used in C)
        struct AlwaysEqual;
        fn unit_structs() {
            let subject = AlwaysEqual;
        }
        // structs can also store references to data owned by something else but this requires lifetimes. (chpt 10)
    }

    fn example_using_structs() {
        struct Rectangle {
            width: u32,
            height: u32,
        }
        fn area() {
            let width1 = 30;
            let height1 = 50;
            println!(
                "area of rectangle is {} square pixels.",
                area1(width1, height1)
            );

            // better way using tuples
            let rect1 = (30, 50);
            println!("area of rectangle is {} square pixels.", area2(rect1));

            // even better way using structs
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
        }
        fn area1(width: u32, height: u32) -> u32 {
            width * height
        }
        fn area2(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
        fn area3(rectangle: &Rectangle) -> u32 {
            //immutable borrow (this way fn area will retain ownership)
            rectangle.width * rectangle.height
        }
    }

    fn method_syntax() {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        // define functions in the context of Rectangle struct
        // methods inside impl block are called associated functions
        // it is possible to have multiple impl blocks for a single struct/type but there is no difference between having it as one impl block
        impl Rectangle {
            // first parameter is always self. this represents the instance of the struct the method is being called on
            fn area(&self) -> u32 {
                // &self is shorthand for self: &Self
                // method borrows Self instance
                self.width * self.height
            }
            fn width(&self) -> bool {
                // this method has same name as one of the properties of the rectangle Struct
                // return true if width greater than zero
                self.width > 0
            }
            // rules about ownership, borrows and mutability still apply here
            fn can_hold(&self, other: &Rectangle) -> bool {
                // &Rectangle is immutable because we only need to read
                // other is a reference because we don't want to transfer ownership
                self.width > other.width && self.height > other.height
            }
            // associated function without self parameter (often used for constructors)
            // to call this we use Rectangle::square(3); since the funciton is namespaced by the struct but does not act as a method on an existing instance of the struct
            fn square(size: u32) -> Self {
                Self {
                    width: size,
                    height: size,
                }
            }
        }
        area_method_example();
        methods_with_more_parameters();
        fn area_method_example() {
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
            println!("area of rectangle is {}", rect1.area());
            if rect1.width() {
                println!("the rectangle has a non zero width, {}", rect1.width);
            }
        }
        /*
         // what about . and -> in c/c++?
         . calls a method on the object directly
         -> calls a method on a pointer to the object, the pointer needs to be dereferenced first. (eg if object is a ptr, object->something() is equivalent to (*object).something())
         // rust does not have -> operator!
         rust has automatic referencing and dereferencing for calling methods.
         when you call object.something() rust automatically adds &, &mut, or * so that object matches the signature of the method.
         so p1.distance(&p2); and (&p1).distance(&p2); are the same
         this is possible becuase methods have a clear receiver with the type of self and whether the method is reading (&self) or mutating (&mut self) or consuming (self)
        */
        fn methods_with_more_parameters() {
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };
            let rect2 = Rectangle {
                width: 10,
                height: 40,
            };
            let rect3 = Rectangle {
                width: 60,
                height: 45,
            };
            // &rect2 is an immutable borrow
            println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
            println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

            let sq = Rectangle::square(3);
        }
    }
}
