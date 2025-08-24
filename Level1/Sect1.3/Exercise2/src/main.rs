// Exercise 2 Requirements:
// ------------------------
// Write a program that calculates the surface of a triangle with one 90 degree angle.
// The formula is half the height multiplied by the base.
// The program should take an input from the user (base & height), and output the result.
//
// Formula: area = (base * height) / 2

use std::io;

fn main() {
    // Print program description
    println!("Triangle Area Calculator (Right Triangle)");
    println!("==========================================");
    
    // Get base from user
    println!("Enter the base of the triangle:");
    let mut base_input = String::new();
    io::stdin()
        .read_line(&mut base_input)
        .expect("Failed to read line");
    
    let base: f64 = base_input
        .trim()
        .parse()
        .expect("Please enter a valid number for base");
    
    // Get height from user
    println!("Enter the height of the triangle:");
    let mut height_input = String::new();
    io::stdin()
        .read_line(&mut height_input)
        .expect("Failed to read line");
    
    let height: f64 = height_input
        .trim()
        .parse()
        .expect("Please enter a valid number for height");
    
    // Calculate area
    let area = (base * height) / 2.0;
    
    // Output the result
    println!("\nResults:");
    println!("--------");
    println!("Base: {}", base);
    println!("Height: {}", height);
    println!("Area of the triangle: {}", area);
}
