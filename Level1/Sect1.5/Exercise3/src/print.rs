// Exercise 3 - Section 1.5 (Print Module)
// ----------------------------------------
// This is the second source file (equivalent to Print.c)
// It contains the print() function that multiplies i by 2 and prints it

// Function that multiplies i by 2 and prints it
pub fn print(i: i32) {
    let result = i * 2;
    println!("Value after multiplying by 2: {}", result);
}
