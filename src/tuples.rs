// Tuples in rust group together values of different type
// Max 12 elements are permitted in a tuple
pub fn run() {
    let person: (&str, &str, i8) = ("UV", "Montreal", 22);
    println!("{} is from {} and is {}", person.0, person.1, person.2)
}