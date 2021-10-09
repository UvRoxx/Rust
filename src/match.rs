/*
Match provides functionality like switch or select(case)

Everymatch is possible and the case must be addressed so match is often combined with enum

default case or else here is indicated with _

*/
pub fn one_two_or_something_else(number: i32) {
    match number {
        1 => println!("This is ONE"),
        2 => println!("This is TWO"),
        // Passing a Else/Default here
        _ => println!("This is something else"),
    }
}

fn match_balance() {
    let balance = 1000;
    match balance {
        0 => println!("Empty!"),
        // This | represents the OR operator so the match will match the value with all the OR
        // conditions specified
        2 | 3 | 5 | 7 | 11 => println!("Prime!"),
        1 | 4 | 6 | 8 | 9 | 10 => println!("Not a Prime!"),
        // Here we a see the range operation shown as .. and the = which specifies a range
        // from ..= to
        12..=1000 => println!("Growing the stash"),
        w if w > 1000 => println!("Wow {},already", w),
        w if w < 0 => println!("In the red!"0),
        // This is our last or/else condition
        _ => println!("Invalid Balance"),
    }
}