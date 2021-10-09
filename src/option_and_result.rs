/*
Rust doesnt have nulls , instead we have Optional types to handle the null reltaed cases
<T> Specifying T means out option can wrap any type in it and so by behaviour isnt type sensitive
*/
pub fn testing_options() {
    enum Option<T> {
        None,
        Some(T),
    }
    let x = Some(7);
    let y = None;
}

// Exceptions do not exist in rust and instead we use Result Type
// Calling unwrap on a value that can by any contain None will cause the thread to panic and the
// execution to end, this is why its is very unsafe to use unwrap in production code
pub fn testing_result() {
    fn give(drink: Option<&str>) {
        match drink {
            Some(inner) => println!("{}? HOw nice", inner),
            None => println!("No Drink? Oh Well..."),
        }
    }
    fn give_vegan(drink: Option<&str>) {
        let inside = drink.unwrap();
        if inside == "milk" { panic!("Police!!"); }

        // Checking for None is a safe way to use unwrap in rust
        if drink == None { println!("What no drink") } else {
            println!("I love {}", inside.unwrap());
        }
    }
    fn start() {
        give("water");
        give(None);

        let milk = Some("milk");
        let nothing = None;

        give_vegan(Some("water"));
        give_vegan(milk);
        give_vegan(nothing);
    }
    start();
}