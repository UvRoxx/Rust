/*
Primitive Types--
Integers: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128:
    The number represents the number of bits they take
    u means they are unsigned / They dont have negative values
Floats: f32,f64
Boolean (bool)
Characters (char)
Tuples
Arrays - Arrays in rust have a fixed size and cannot grow dynamically
         Vectors are present in rust to overcome this limitation
Note: Rust is a statically typed language, which means that it must know the types of all the
variables at compile time, however the compiler can usually infer what type we want to use based
on the type of value we use.

tl:dr - We can have variables without specifying types, specify types to optimize performance.
*/

pub fn run() {
// Default for numbers is i32
    let x = 1;

// Default for float is f64
    let y = 2.5;

// Explicit type
    let z: i64 = 454545455454;
// Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

// Boolean
    let is_active: bool = true;
    println!("{:?}", (x, y, z, is_active));

// Getting boolean from an expression
    let is_greater = 10 > 5;
    println!("Is Greater {}", is_greater);

// Chars/Unicode in rust are specified using '' , using "" will imply that its a string
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{} {}", a1, face);
}