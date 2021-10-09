// Conditionals - Used to check the condition of something and act on the result
pub fn has_id(id: bool) {
    if id {
        println!("The person has an ID");
    } else {
        println!("The person doesnt have an ID");
    }
}

pub fn run() {
    let person: (&i32, &bool) = (&19, &true);
    // If/Else
    // Note: In Rust It is Not Possible to compare a borrowed integer to a literal integer
    // cannot compare i32(literal) and &i32(borrowed)
    // https://stackoverflow.com/questions/40677086/why-isnt-it-possible-to-compare-a-borrowed-integer-to-a-literal-integer
    if person.0 > &21 {
        println!("Age is greater than 21");
    } else {
        println!("Pass")
    }
    println!("Check Age says:{}", check_age(20));
}
