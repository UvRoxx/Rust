// Variables in rust hold primitive data or references to data
// Variables are immutable by default i.e All variables by default are const
// Variables can be made mutable by adding "mut" in front of the var
// Rust is a block scoped language

pub fn run() {
    // Immutable Variables
    let name = "Utkarsh";
    let age = 22;
    // Mutable Variables
    let mut mutable_age = age;
    println!("My name is {} I'm {}", name, age);
    mutable_age += 1; // Value is mutated/incremented
    println!("My name is {} I'm {}", name, mutable_age);
    println!("Reference Age {}", age);

// We can also define constants in rust but when defining constants we
// need to explicitly define type and use upper case for best-practice

    //     Constant Variables
    const ID: i32 = 001;
    println!("Constant ID is {}", ID);

//     Multiple variable assignment
    let (my_name, my_age) = ("UV", 22);
    println!("{} is {}", my_name, my_name);
}