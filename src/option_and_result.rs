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
        // When passing multiple conditons that can cover the same input in this case
        // Condition - 1 and Condition-2 are both valid for input milk, in this case the only the
        // first condition specified will get executed
        // This will raise warning , this is called "unreachable patten"
        match drink {
            Some("milk") => println!("Hey this is Milk"),//condition 1
            Some(inner) => println!("{}? HOw nice", inner),// condition 2
            None => println!("No Drink? Oh Well..."),
        }
    }
    fn give_vegan(drink: Option<&str>) {

        // Using Match to achieve the similar result as unwrap
        match drink {
            Some("milk") => println!("Hey this is milk"),
            None => println!("No Drink? Oh Well..."),
            _ => println!("What is this drink"),
        }

        // Using Unwrap in an unsafe manner
        let inside = drink.unwrap();
        if inside == "milk" { panic!("Hey Thats Milk!!"); }

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

// Result is Also an enum but instead of wrapping in a Some
// result always wraps a value in Ok or Err
pub fn testing_options_more_further() {
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    let x = Ok(7);
    let y = Err("Error");
}
