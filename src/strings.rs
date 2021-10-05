// In rust we have 2 kinds of strings mainly
// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data-structure-To use when we need to modify or own string data
pub fn run() {

// Immutable fixed length
    let hi = "Hi";

// Growable String, mut declaration is necessary for declaring any mutable data type
// This is used in conjunction with String::from
    let mut hello = String::from("Hello");
    println!("{}", hello);

// Get length of the given strings
// str.len() works for both string types in the same fashion
    println!("Length of Hi :{}", hi.len());
    println!("Length of Hello :{}", hello.len());

// push method
// This method pushes an new element at the end of the  string
// Only works with ch
    hello.push('.');

//  push_str
//  This method adds a string to the mutable string
    hello.push_str("World");

    println!("{}", hello);

//  Capacity in bytes
    println!("Capacity: {}", hello.capacity());

//  Checking if empty
    println!("Is empty:{}", hello.is_empty());
//  Contains
    println!("Contains 'World' {}", hello.contains("World"));
//  Replace
    println!("Replace :{}", hello.replace("World", "There"));

//  Looping through string by whitespace
    for word in hello.split('.') {
        println!("{}", word);
    }

// Creating string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

// Assertion testing
// These errors are only raised when the condition doesnt go through
    asset_eq!(2,slen());
}