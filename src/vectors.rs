// Vectors are arrays that supports a dynamic size
// While the size is dynamic, the elements won't be mutable unless we specify mut



use std::mem;

pub fn run() {
    // Vectors
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Debug Tray
    println!("{:?}", numbers);
    // All operations between arrays and vector share a common signature but push, pop and other
    // size mutating methods are limited to vectors only
    numbers.push(6);
    println!(" After Push{:?}", numbers);
    numbers.pop();
    println!(" After Pop{:?}", numbers);
}