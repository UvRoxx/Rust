// pub defines that out function is public
pub fn run() {
    // Print to console
    // Semicolons are REQUIRED in rust
    println!("Hello from print.rs");

    // In rust string literal formatting is very important
    // We cannot print variable data types directly in
    // the println! they must passed to the string literal
    println!("This is 2+2 : {}", (2 + 2));

    //Basic Formatting
    // When using multiple place holders the insertion always follows the order unless specified
    println!("This is {} {}", "basic", "formatting");

    // Positional Arguments
    println!(
        "{0} is from {1} and  {0} likes to {2}",
        "UV", "Montreal", "Code"
    );

    // Named Arguments
    println!(
            "{name} is a cool {type}",
        name="Rust",
        type="Language",
    );
    // Placeholder traits
    println!("Binary:{:b} Hex:{:x} Octal:{:o}", 10, 10, 10);

//     Place holder for debug trait
//     Here we can wrap all the values we need to print in a tuple
//     then use pass it along with "{:?}" in the println!
    println!("{:?}", (12, true, "hello"));

//     Basic Math
    println!("10+10 ={}", 10 + 10);
}
