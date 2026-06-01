fn main() {
    // Variables are normally stored as immutable, but can be declared immutable with mut
    // IMMUTABLE
    let x = 5;
    println!("The value of x is: {x}");
    //x = 6; // since x is currently immutable, x CANNOT reassign to 6
    //println!("The value of x is: {x}");

    // MUTABLE
    let mut x = 5; // mut marks the variable x as mutable
    println!("The value of x is: {x}");
    x = 6; // since x is currently mutable, x CAN be reassigned to 6
    println!("The value of x is: {x}");

    // Consts are always immutable, declared with const and must be annotated(e.g. u32)
    // also consts typically use  UPPER_SNAKE_CASE
    // They are valid throughout the entire program within the scope declared
    // CONSTS
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // A variable can go into a scope and can change within the scope,
    // but the variable outside the scope remains unchanged
    // so x = 5, new scope{ x = x + 1 = 6 }: within scope x = 6, outside scope x still equals 5
    // SHADOWING
    let x = 5; // sets x to 5
    let x = x + 1; // sets x to 5 + 1 = 6
    {
        let x = x * 2; // sets x to 6 * 2 = 12: within this scope{ } this.x shadows the outer.x
        println!("The value of x in the inner scope is: {x}"); // prints x as 12
    }
    println!("The value of x is: {x}"); // prints x as 6, since this x remained outside the scope
                                        // of the inner x

    // DATA TYPES
    // data types can sometimes be inferred based on what is being done, but sometimes it could
    // be too ambiguous so here is an example that needs type annotation
    let guess: u32 = "42".parse().expect("Not a number!"); // here we say the type will change
                                                           // to a u32
    println!("{guess}");

    // SCALAR TYPES
    // A scalar types in represented by a single value. Ex. integers, floating-point numbers,
    // booloans, and characters.
    //
    // INTEGER TYPES
    // Number with no fractional component(u32, i32)
    // Ex. (i8, u8 | i16, u16 | i32, u32 | i64, u64 | i128, u128 | isize, usize |
    // integer literals can be in any of the forms above also could be represented with 
    // a suffix such as 57u8. Number literals can use an '_' to make it easier to read
    // large numbers 1_000_000 = 1000000
    //Number literals	Example
    //Decimal	98_222
    //Hex	0xff
    //Octal	0o77
    //Binary	0b1111_0000
    //Byte (u8 only)	b'A'
    //
    // FLOATING-POINT TYPES
    // f32 and f64
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {x}, y: {y}");

    // Numeric Operations
    // addition;+, subtraction;-, multiplication;*, division;/, remainder;%
    let sum = 5 + 10; // sum = 15
    let difference = 95.5 - 4.3; // difference = 91.2
    let product = 4 * 30; // product = 120
    let quotient = 56.7 / 32.2; // quotient = 1
    let truncated = -5/3; // truncated = -1
    let remainder = 43 % 5; // remainder = 3
    println!("sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, truncated: {truncated}, remainder: {remainder}");

    // THE BOOLEAN TYPE
    // true | false : 1 byte in size
    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");

    // THE CHARACTER TYPE
    // Simple letters by them selves, also note that single quotes(') were used
    // 4 bytes in size
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");

    // COMPOUND TYPES
    // Able to group multiple values into one type: tuples and arrays
    //
    // THE TUPLE TYPE
    // Groups different type values into one compound type, fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // <= destructuring
    println!("The value of x is: {x}, y is: {y}, z is: {z}");
    // Access the individual element with a . followed by the index
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");
    //
    // THE ARRAY TYPE
    // Arrays only hold one type, have a fixed length
    let a = [1, 2, 3, 4, 5]; // cannot grow or shrink because it lives in stack memory
                             // vectors are similar but live in heap so they can grow or shrink
    println!("a: {:?}", a);
    // Arrays are better used when you know the amount of elements being used
    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
    println!("{:?}", months);
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // i32 is type, 5 is the number of elements
    println!("a: {:?}", a);
    let a = [3; 5]; // Sets a = [3, 3, 3, 3, 3]
    println!("a: {:?}", a);
    let a = [1, 2, 3, 4, 5];
    let first = a[0]; // first = 1
    let second = a[1]; // second = 2
    println!("first: {first}, second: {second}");

    // FUNCTIONS
    // Denoted by fn and uses all lowercase and underscores separating words
    // define location is not too important, but the caller must see the scope
    fn another_function() { println!("Another function."); }
    another_function();
    //
    // PARAMETERS
    // Special variables that are part of a fn's signature
    fn another_function_with_parameters(x: i32) { println!("The value of x is: {x}"); }
    // note that x's type is annotated, this is required
    another_function_with_parameters(5);
    fn print_labeled_measurement(value: i32, unit_label: char) { println!("The measurement is : {value}{unit_label}"); }
    print_labeled_measurement(5, 'h');
    // note the commas used to separate the variables
    //
    // STATEMENTS AND EXPRESSIONS
    // Statements are instructions that perform some action and do not return a value
    let y = 6;
    println!("The value of y is : {y}");
    // Expressions evaluate to a resultant value
    // Calling a function, a macro or { }, are expressions
    let y = {
        let x = 3;
        x + 1 // since the ; is used here it is returned out
    };
    println!("The value of y is : {y}");
    // 
    // FUNCTIONS WITH RETURN VALUES
    // Functions can return a value
    //         v  return is denoted by the ->
    fn five() -> i32 { 
        5 // note no ; nor a return keyword, the last line in a function will return
    }
    let x = five();
    println!("The value of x is: {x}");
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    let x = plus_one(5);
    println!("The value of x is: {x}");

    // COMMENTS
    // You have seen them all throughout this program using // makes everything behind it a comment

    // CONTROL FLOW
    //
    // if EXPRESSIONS
    // Allows one to branch code depending on conditions.
    // If this true run this, else run that
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // The condition must be a bool. So number < 5 is equal to true|false
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }
    // HANDLING MULTIPLE CONDITIONS WITH else if
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // USING if IN A let STATEMENT
    // if is an expression, so we can use it on the right side
    let condition = true;
    let number = if condition { 5 } else { 6 }; // not 5 and 6 are the same type
    println!("The value of numbber is: {number}");
    //
    // REPETITION WITH LOOPS
    // Code that is repeatedly executed
    //
    // REPEATING CODE WITH LOOP
    //loop { println!("again!"); } // this is commented out cause this would cause in infinite loop
    // break can break the loop and continue can restart the loop
    //
    // RETURNING VALUES FROM LOOPS
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    //
    // DISAMBIGUATING WITH LOOP LABELS
    // break and continue specifies to the innermost loop at that point
    // although labels can specify which one
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 { break; }
            if count == 2 { break 'counting_up; }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    //
    // STREAMLINING CONDITIONAL LOOPS WITH while
    // Loops while a statement is true, breaks when false
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
    //
    // LOOPING THROUGH A COLLECTION WITH for
    // for can be used to index for each item in a collection
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
    // range(x..y) can be used with for
    //              vv  .rev() is reverse
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
