fn main() {
    // Ownership is a set of rules that govern how a Rust program manages memory.
    // This creates safe and efficient code
    //
    // THE STACK AND THE HEAP
    // The stack stores values in the order received, then removes the values in the opposite order
    // Last in, first out (LIFO)
    // Adding data is called pushing to the stack, removing is popping off the stack
    // The data must have a known fixed size, else it will be stored in the heap
    // The heap is less organized, when requesting space, the memory allocator finds an empty spot,
    // marks it as being used and returns a pointer(address of the location): proccess is called
    // allocating on the heap. The pointer is of fixed size and knows so it gets added to the stack
    // 
    // Pushing to the stack is faster than allocating on the heap because it never has to search
    // for a place.
    //
    // OWNERSHIP RULES
    // Each value in Rust has an owner
    // There can only be one owner at a time
    // When the owner goes out of scope, the value will be dropped.
    //
    // VARIABLE SCOPE
    // Scope is where something like a variable is valid a range so to speak
    {  // above this { is outside the scope and s is not valid, since it is not declared yet
        let s = "hello"; // s is valid from here forward
        println!("{s}");
    } // this scope is now over, and s is no longer valid
    //
    // THE STRING TYPE
    // There are string literals which is a hardcoded string value ex. let s = "hello";, these are
    // stored in the stack
    // There is also the String type which can have an unknown size and thus this type manages data
    // allocated on the heap.
    // You can create a String from a string literal using the from function
    let s = String::from("hello");
    println!("{s}");
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // this will print 'hello, world!'
    // So why can String be mutated but literals cannot
    //
    // MEMORY AND ALLOCATION
    // Since a String is mutable or growable an amount of memory is allocated to the heap
    // 1. Memory must be requested from the memory allocator
    // 2. A way tor return this memory to the allocator when done with String
    // One is acheived by us with String::from
    // Two is taken care of by Rust by returning the memory once it goes out of scope
    {  // above this { is outside the scope and s is not valid, since it is not declared yet
        let s = "hello"; // s is valid from here forward
        println!("{s}");
    } // this scope is now over, and s is no longer valid
    // This is done with Rust's drop function which is called automatically at the '}'
    //
    // VARIABLES AND DATA INTERACTING WITH MOVE
    // Miltiple variables can interacte with the same data in different ways
    let x = 5;
    let y = x;
    println!("x: {x}, y: {y}");
    // Set x = 5, then set y not equal to x but equal to what x equals. So y = 5
    // This happens because integers are simple values with a fixed size, therefore they are push
    // to the stack
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("s1: {s1}, s2: {s2}"); // this will not work because the pointer is moved to s2
    //s1 no longer exists
    println!("s2: {s2}");
    // The same does not happen for the above. String work a little differently under the hood
    // A string has 3 parts, a pointer to the memory that holds the contents, a length, and a
    // capacity. These 3 parts are stored on the stack(left) and it points to the memory on the
    // heap(right)
    // _________s1_________  _________________
    // |  name    | value |  | index | value |
    // | pointer  | -------->|   0   |   h   |
    // |  length  |   5   |  |   1   |   e   |
    // | capacity |   5   |  |   2   |   l   | 
    // --------------------  |   3   |   l   |
    //                       |   4   |   o   |
    //                       -----------------
    //  The below is the representation of what is actually happening. Another copy is not made.
    // _________s1_________   _________________
    // |  name    | value |   | index | value |
    // | pointer  | --------->|   0   |   h   |
    // |  length  |   5   | | |   1   |   e   |
    // | capacity |   5   | | |   2   |   l   | 
    // -------------------- | |   3   |   l   |
    //                      | |   4   |   o   |
    //                      | -----------------
    // _________s2_________ |
    // |  name    | value | | Since s1 is just where the data is, s2 becomes the same.
    // | pointer  | -------- and s1 is dump out as it becomes out of scope/there can only be one
    // |  length  |   5   |  owner
    // | capacity |   5   |  
    // -------------------- 
    //
    // SCOPE AND ASSIGNMENT
    // The inverse is also true for the relationship between scoping, ownership, and memory being
    // freed via the drop function. When you assign a new value to an existing variable Rust will
    // call drop and free the original value's memory
    let mut s = String::from("hello");
    println!("{s}, world!");
    s = String::from("ahoy");
    println!("{s}, world!");
    // Since the old "hello" is being overwritten Rust drops the old value in memory
    //
    // VARIABLES AND DATA INTERACTING WITH clone
    // If one does not want to deeply copy the heap data of the String, one can use clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    // This will copy the heap data instead of the stack data.
    //
    // STACK-ONLY DATA: copy
    // Here is a contradiction
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
    // This works in the sense where x is not dumped but y = x.clone();. The reason the .clone() is
    // not needed is because integers have a known size at compile time, therefore stored onto the
    // stack.
    // Now Rust has a special annotation called Copy trait that one can place on types stored in
    // the stack. If a type usus the Copy trait, variables that use it do not move, but rather are
    // trivially copied, making them valid after the assignment to another variable.
    // Rust won't let you annotate a type with Copy if the type has implemented the Drop trait.
    // As a general rule any group of simple scalar values can implement Copy, and nothing the
    // requires allocation or is some form of resource can implement Copy
    // Ex.
    // All integer types, such as u32
    // Boolean type, bool:true|false
    // All floating-point types, f64
    // Character type, char
    // Tuples, if they only contain types that also implement Copy,
    // Works:(i32, i32) Bad:(i32, String)
    //
    // OWNERSHIP AND FUNCTIONS
    // Pass a variable to a function will move or copy, just as assignment does.
     let s = String::from("hello"); // s comes into scope
     takes_ownership(s); // s's value moves into the function...
                         // ... and so is no longer valid hear
     let x = 5; // x comes into scope
     
     makes_copy(x); // Because i32 implements the Copy trait,
                    // x does Not move into the function, so it's
                    // okay to use x afterward
    // When '}' is reached at the end of this current function x goes out of scope, then s. However,
    // because s's value was moved, nothing special happens.
    // Using s after the takes_ownership, Rust would throw a compile error.
    //
    // RETURN VALUES AND SCOPE
    // Return values can also transfer ownership
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves
                                       // its return value into s3
    // When '}' is reached at the end of this current function s3 goes out of scope and is dropped.
    // s2 was moved, so nothing happens. s2 goes out of scope and is dropped
    //println!("s1: {s1}, s2: {s2}, s3: {s3}"); // this will not work since s2 is dropped
    println!("s1: {s1} s3: {s3}");
    // This works but what if the function must use the value but not take ownership of it. It is
    // annoying that anything we pass in also needs to be passed back if we want to use it again.
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
    // References and borrowing will take care of this
    //
    // REFERENCES AND borrowing
    // Instead we can send a reference to the String value. A reference is like a pointer in that
    // it's an address we can follow to access the data stored at the address. Also a reference is
    // guaranteed to point to a valid value for the like of that reference.
    let s1 = String::from("hello");
    let len = calculate_length_reference(&s1);
    println!("The length of '{s1}' is {len}.");
    // The '&' is used to allow you to refer to some value rather than take ownership of it.
    // _________s_________  _________s1_________  _________________
    // |  name   | value |  |   name   | value |  | index | value |
    // | pointer | -------->| pointer  | -------->|   0   |   h   |
    // -------------------  |  length  |   5   |  |   1   |   e   |
    // ^calc_len_referere^  | capacity |   5   |  |   2   |   l   |
    //                      --------------------  |   3   |   l   |
    //                                            |   4   |   o   |
    //                                            -----------------
    //  This is called borrowing. Although you cannot modify something you are borrowing.
    let s = String::from("hello");
    change(&s);
    //
    // MUTABLE REFERENCES
    // Now it is possible to allow us to modify a borrowed value with a few tweaks
    let mut s = String::from("hello");
    change_mut(&mut s);
    // Now there is a caveat. If you have a mutable reference to a value, you can have no other
    // references to that value.
    let mut s = String::from("hello");
    //let r1 = &mut s;
    let r1 = "s";
    let r2 = &mut s;
    println!("{r1}, {r2}");
    // This is because we cannot borrow a mutable more than once at a time
    // Now {} can be used to create a new scope, allowing for multiple mutable references, just not
    // simultaneous owns
    let mut s = String::from("hello");
    { let r1 = &mut s; println!("{r1}"); } // since r1 goes out of scope we can make a new reference
    let r2 = &mut s;
    println!("{r2}");
    // Rust enforces a similar rule for combining mutable and immutable references.
    //let mut s = String::from("hello"); // this is the actual code needed to represent belowe
    let s = String::from("hello"); // this is here just to remove lint errors
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    //let r3 = &mut s; // BIG problem
    //println!("{r1}, {r2}, and {r3}");
    println!("{r1}, {r2}");
    // Now this will work.
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1}, {r2}");
    let r3 = &mut s; // BIG problem
    println!("{r3}");
    // Since r1 and r2 are no longer used after the println the compiler recognizes it is no longer
    // being used.
    // 
    // DANGLING REFERENCES
    // A dangling pointer is when a pointer references a location in memory that may have been
    // given to someone else by freeing some memory while preserving a pointer to that memory. Rust
    // will guarantee that references never go out of scope before the reference to the data does.
    let reference_to_nothing = dangle();
    println!("{reference_to_nothing}");
    //
    // THE RULES OF REFERENCES
    // At any give time, you can have either one mutable reference or any number of immutable
    // references
    // Reference must always be valid
    //
    //
    // THE SLICE TYPE
    // Slice is a kind of reference, and lets you reference a contiguous sequence of elements in a
    // collection
    // Here is a problem to solve. Write a function that takse a string of words separated by
    // spaces and returns the first word, or if no spaces are found then return the whole string.
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    println!("{word}");
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but s no longer has any content that we could meaningfilly
    // use with the value 5, so word in now totally invalid!
    //
    // STRING SLICES
    // A string slice is a reference to a contiguous sequence of the elements of a String
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}");
    // _________s_________     _________________
    // |   name   | value |    | index | value |
    // | pointer  |   -------->|   0   |   h   |
    // |  length  |   11  |    |   1   |   e   |
    // | capacity |   11  |    |   2   |   l   |
    // -------------------     |   3   |   l   |
    // _______world_______     |   4   |   o   |
    // |  name   | value |     |   5   |       |
    // | pointer |   --------->|   6   |   w   |
    // | length  |   5   |     |   7   |   o   |
    // -------------------     |   8   |   r   |
    //                         |   9   |   l   |
    //                         |  10   |   d   |
    //                         -----------------
    // With Rust's .. range syntax, if you want to start at 0, you do not have to place 0 before the ..
    let s = String::from("hello");
    let slice = &s[0..2];
    println!("{slice}");
    let slice = &s[..2];
    println!("{slice}");
    // The same can be done with the last index
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    println!("{slice}");
    let slice = &s[3..];
    println!("{slice}");
    // The same can be done with both at the same time
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    println!("{slice}");
    let slice = &s[..];
    println!("{slice}");
    let mut s = String::from("hello world");
    let word = first_word_slice(&s); // word will get the value 5
    println!("{word}");
    s.clear(); // this empties the String, making it equal to ""
    //
    // STRING LITERALS AS SLICES
    let s = "Hello, world!";
    println!("{s}");
    // s here is &str: it is a slice pointing to that specific point of the binary. This is why
    // string literals are immutable; &str is an immutable reference
    //
    // STRING SLICES AS PARAMETERS
    // Since you can take slices of literals and String values you can change the
    // first_word_slice(s: &String) -> &str { to first_word_slice(s: &str) -> &str {
    // This allows us to pass a string slice, or a reference to the String
    let my_string = String::from("hello world");
    // `first_word' works on slices of `String` s, whether partial or whole.
    let word = first_word_reference(&my_string[0..6]);
    println!("{word}");
    let word = first_word_reference(&my_string[..]);
    println!("{word}");
    // `first_word` also works on references to `String`s, which are equivalent to the whole
    // slices of `String`s
    let word = first_word(&my_string);
    println!("{word}");
    let my_string_literal = "hello world";
    // `first_word` works on slices of stirng literals, whether partial or whole.
    let word = first_word_reference(&my_string[0..6]);
    println!("{word}");
    let word = first_word_reference(&my_string[..]);
    println!("{word}");
    // Because string literals *are* string slices already, this works too, without the slice
    // syntax!
    let word = first_word_reference(my_string_literal);
    println!("{word}");
    //
    // OTHER SLICES
    // String slices are specific to stirngs, but arrays can use slices too
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

    // OWNERSHIP AND FUNCTIONS
fn takes_ownership(some_string: String) { // some_string comes into scope
         println!("{some_string}");
} // Here' some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
         println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

    // RETURN VALUES AND SCOPE
fn gives_ownership() -> String { // gives_ownership will move its return value into the funciton
                                 // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    println!("{some_string}");
    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

    // RETURN VALUES AND SCOPE
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() return the length of a String
    (s, length)
}

    // REFERENCES AND borrowing
//fn calculate_length_reference(s: &String) -> usize { // s is a reference to a String this is the
//actual code, it is commented out just to remove lint errors
fn calculate_length_reference(s: &str) -> usize { // s is a reference to a String
    s.len() 
} // Here, s goes out of scope. But because s does not have ownership of what it refers t, the
  // String is not dropped.
fn change(some_string: &String) {
    //some_string.push_str(", world");
    println!("{some_string}");
}
fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

    // DANGLING REFERENCES
//fn dangle() -> &String {
fn dangle() -> String {
    let s = String::from("hello");

    //&s // we could try to return a reference to the String, but the string s will be dropped so it would
    //no longer exist, Rust well prevent us from doing this. So we must return s instead.
    s
}

    // THE SLICE TYPE
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return i; }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }
    &s[..]
}

fn first_word_reference(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[0..i]; }
    }
    s
}
