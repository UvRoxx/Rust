use std::collections::HashMap;

fn run() {
    let mut marks = HashMap::new(); //This will create a new empty hash map
    marks.insert("this_is_a_cool_key", 69);//Insert is how we add value onto the hashmap using the key and value
    marks.insert("cool", 22);//Insert is how we add value onto the hashmap using the key and value
    println!("This is the map {:?}", marks);
    println!("Length of the hashmap {}", marks.len());

    // Pulling the value from the dict
    match marks.get("cool") {
        Some(mark) => println!("You got {}", mark),
        None => println!("Value Not Found"),
    }
    //Removing A Value
    marks.remove("cool");

    // Looping through the Hashmap
    for (key, value) in &marks {
        println!("This are the {} Key and {} Value", key, value);
    }
    //Check if a key exist in a map
    println!("Does this value exist {}", marks.contains_key("new_key"));
}
