// Arrays in rust are lists with Fixed length and all elements are of the same data type
// Arrays are defined as let name_of_array : [data_type,length] = [content];
// Note: Arrays Must always contain the number of elements in content as specified in length
// Arrays can be made mutable by adding the mut before the array name

use std::mem;

pub fn run() {
    // Static Array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // Looping through the array
    for n in numbers {
        println!("{}", n);
    }
    // Debug Tray
    println!("{:?}", numbers);

    // Getting Single Val
    println!("{}", numbers[0]);

    // Mutable Array
    let mut cool_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", cool_numbers);
    cool_numbers[2] = 5;
    println!("After Mutation:{:?}", cool_numbers);

    // Arrays are stack allocated
    println!("This Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    // In  rust we use the range operator as from_index .. to_index
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}