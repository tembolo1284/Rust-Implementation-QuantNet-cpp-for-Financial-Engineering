// Exercise 4 Requirements:
// ------------------------
// Create a C-program that uses the fact that 0 (zero) is interpreted as FALSE
// and non-zero is interpreted as TRUE. The C-program can be made easier to read
// when this 0 (or non-zero) is assigned to a variable e.g. an int called married.
// Use the ?: operator to print if someone is married or not.
// (See if you can use a single printf)
//
// Note: Rust uses bool type (true/false) instead of int for booleans
// The ?: operator in C is replaced by if/else expressions in Rust

use std::io;

fn main() {
    println!("Marriage Status Checker");
    println!("=======================");
    
    // In C, we'd use int with 0/1. In Rust, we can do both approaches:
    
    // Approach 1: Using integer like C (0 = false, non-zero = true)
    println!("\nApproach 1: Using integer (C-style)");
    println!("Enter 0 for not married, any other number for married:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let married_int: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number");
    
    // Single println! using if/else expression (Rust's equivalent of ?: operator)
    println!("Person is {}", if married_int != 0 { "married" } else { "not married" });
    
    // Approach 2: More idiomatic Rust using bool
    println!("\nApproach 2: Using boolean (Rust-style)");
    println!("Are you married? (yes/no):");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let married: bool = input.trim().to_lowercase() == "yes";
    
    // Single println! with inline if/else expression
    println!("Person is {}", if married { "married" } else { "not married" });
    
    // Demonstrating multiple cases with hardcoded values
    println!("\nDemonstration with hardcoded values:");
    
    let person1_married = 0;  // Not married (0 = false)
    let person2_married = 1;  // Married (non-zero = true)
    let person3_married = 42; // Also married (non-zero = true)
    
    println!("Person 1 (value={}): {}", 
             person1_married, 
             if person1_married != 0 { "married" } else { "not married" });
    
    println!("Person 2 (value={}): {}", 
             person2_married,
             if person2_married != 0 { "married" } else { "not married" });
    
    println!("Person 3 (value={}): {}", 
             person3_married,
             if person3_married != 0 { "married" } else { "not married" });
}
