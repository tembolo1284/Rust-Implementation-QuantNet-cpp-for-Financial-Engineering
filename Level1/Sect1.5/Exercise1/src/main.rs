// Exercise 1 - Section 1.5 Requirements:
// ---------------------------------------
// Write a C-program that calls a function minus().
// This function receives two arguments and returns the difference
// (regular subtraction, not absolute). This difference should be
// printed on screen.

use std::io;

// Function that performs subtraction (equivalent to C's minus function)
fn minus(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    println!("Subtraction Function Demo");
    println!("=========================");
    
    // Get first number from user
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num1: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    // Get second number from user
    println!("Enter the second number:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num2: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    // Call the minus function and store result
    let result = minus(num1, num2);
    
    // Print the result
    println!("\n╔════════════════════════════════════════╗");
    println!("║                RESULT                  ║");
    println!("╚════════════════════════════════════════╝");
    println!("{} - {} = {}", num1, num2, result);
    
    // Additional examples with hardcoded values
    println!("\n╔════════════════════════════════════════╗");
    println!("║           MORE EXAMPLES                ║");
    println!("╚════════════════════════════════════════╝");
    println!("minus(10, 3) = {}", minus(10, 3));
    println!("minus(5, 8) = {}", minus(5, 8));
    println!("minus(-4, 3) = {}", minus(-4, 3));
    println!("minus(0, 5) = {}", minus(0, 5));
    
    // Explanation
    println!("\n╔════════════════════════════════════════╗");
    println!("║            FUNCTION INFO               ║");
    println!("╚════════════════════════════════════════╝");
    println!("• Function signature: fn minus(a: i32, b: i32) -> i32");
    println!("• Returns: a - b (regular subtraction)");
    println!("• Note: Result can be negative");
}
