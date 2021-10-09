/*
Rust Follows the concept of ownership where. Here we can pass a val from one function to another
by value basis or by refrence basis.
Passing a direct value can often cause the program to panick as when the inheriting funtion goes
out of scop both the function and the value it inherited are discarded and the parent function now
no longer has access to.
Thats why in rust we have to pass values as refrence types using the & notation before the variable
name.
*/
fn give() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    take(vec);//function give no longer has the ownership of vec
    vec.push(1); //This line will now generate errors as the vec
    // passed to take has now been de-allocated from the memory along with vec
}

pub fn take(vec: Vec<i32>) {
    println!("This is the vector:{}", vec)
}
// & - Borrwing/Refrence
// Borrowing ensures that a varibales passed to another function still reatains ownership of its
// parent function and so it stays in memory until the parent funcition has completed its life-cycle
// Refrences are static as well but if the borrower needs to mutate the value of the
// refrence we pass it as: mut &value_name

fn lender() {
    let mut vec: Vev<i32>::new();
    vec.push(1);
    borrower(&vec);
    vec.push(1);//This time as the value was borrowed its not de-allocated and hence lender
    // still has access to it after  borrower finished its operation
}

fn borrower(vec: Vec<i32>) {
    println!("This is the vector I just borrowed:{}", vec)
}