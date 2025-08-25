// Exercise 3 - Section 1.5 Requirements:
// ---------------------------------------
// Write a program that consists of two source-files.
// The first (Main.c) contains the main() function and gives
// the variable i a value. The second source-file (Print.c)
// multiplies i by 2 and prints it. Print.c contains the
// function print() which can be called from main().
//
// Note: In Rust, we'll use modules to simulate multiple source files

// This is the main module (equivalent to Main.c)

// Declare the print module (equivalent to including Print.c)
mod print;

use std::io;

fn main() {
    println!("Multi-File Program Demo");
    println!("=======================");
    
    // Give the variable i a value
    println!("Enter a value for i:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let i: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║           CALLING print()              ║");
    println!("╚════════════════════════════════════════╝");
    println!("Original value of i: {}", i);
    
    // Call the print function from the print module
    print::print(i);
    
    // Additional examples
    println!("\n╔════════════════════════════════════════╗");
    println!("║          MORE EXAMPLES                 ║");
    println!("╚════════════════════════════════════════╝");
    println!("Calling print(5):");
    print::print(5);
    
    println!("\nCalling print(-3):");
    print::print(-3);
    
    println!("\nCalling print(100):");
    print::print(100);
    
    // Explanation
    println!("\n╔════════════════════════════════════════╗");
    println!("║         PROGRAM STRUCTURE              ║");
    println!("╚════════════════════════════════════════╝");
    println!("• main.rs: Contains main() function");
    println!("• print.rs: Contains print() function");
    println!("• print() multiplies input by 2 and displays it");
    println!("• In C: separate .c files linked together");
    println!("• In Rust: modules in separate .rs files");
}
