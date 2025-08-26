// Exercise 1 - Section 1.7 Requirements:
// ---------------------------------------
// Try to create a function Swap(). This function must exchange the value
// of two variables. For example: if i=123 and j=456, then i=456 and j=123
// after the Swap() function has been called. The variables i and j are
// declared, initialised and printed in the function main().
// This problem can be solved by using pointers as arguments for the Swap() function.
//
// Note: In Rust, we use mutable references instead of pointers

use std::io;

// Swap function using mutable references (Rust's equivalent to pointers)
fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

// Alternative swap using Rust's built-in swap (for comparison)
fn swap_rust_way(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
}

// Another alternative using tuple destructuring
fn swap_tuple(a: &mut i32, b: &mut i32) {
    (*a, *b) = (*b, *a);
}

fn main() {
    println!("Swap Function Demonstration");
    println!("===========================\n");
    
    // Get values for i and j from user
    println!("Enter value for i:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut i: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    println!("Enter value for j:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut j: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    // Print original values
    println!("\n╔════════════════════════════════════════╗");
    println!("║          BEFORE SWAP                   ║");
    println!("╚════════════════════════════════════════╝");
    println!("i = {}", i);
    println!("j = {}", j);
    
    // Call swap function with mutable references
    swap(&mut i, &mut j);
    
    // Print swapped values
    println!("\n╔════════════════════════════════════════╗");
    println!("║           AFTER SWAP                   ║");
    println!("╚════════════════════════════════════════╝");
    println!("i = {}", i);
    println!("j = {}", j);
    
    // Demonstrate with more examples
    println!("\n╔════════════════════════════════════════╗");
    println!("║         MORE EXAMPLES                  ║");
    println!("╚════════════════════════════════════════╝");
    
    let mut x = 123;
    let mut y = 456;
    println!("Before: x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("After:  x = {}, y = {}", x, y);
    
    // Swap back using Rust's built-in
    println!("\nUsing Rust's std::mem::swap:");
    swap_rust_way(&mut x, &mut y);
    println!("After:  x = {}, y = {}", x, y);
    
    // Using tuple destructuring
    println!("\nUsing tuple destructuring:");
    swap_tuple(&mut x, &mut y);
    println!("After:  x = {}, y = {}", x, y);
    
    // Memory address demonstration
    println!("\n╔════════════════════════════════════════╗");
    println!("║      MEMORY ADDRESS EXPLANATION        ║");
    println!("╚════════════════════════════════════════╝");
    
    let mut a = 10;
    let mut b = 20;
    println!("Variables a={} and b={}", a, b);
    println!("Address of a: {:p}", &a);
    println!("Address of b: {:p}", &b);
    
    println!("\nWhen we call swap(&mut a, &mut b):");
    println!("• We pass mutable references (addresses)");
    println!("• The function modifies values at those addresses");
    println!("• Original variables are changed");
    
    swap(&mut a, &mut b);
    println!("\nAfter swap: a={} and b={}", a, b);
    println!("Addresses remain the same:");
    println!("Address of a: {:p}", &a);
    println!("Address of b: {:p}", &b);
    
    // Pointer concepts
    println!("\n╔════════════════════════════════════════╗");
    println!("║      C POINTERS VS RUST REFERENCES     ║");
    println!("╚════════════════════════════════════════╝");
    println!("C Pointers:");
    println!("  void swap(int *a, int *b)");
    println!("  Called with: swap(&i, &j)");
    println!("  Dereference: *a = *b");
    println!();
    println!("Rust References:");
    println!("  fn swap(a: &mut i32, b: &mut i32)");
    println!("  Called with: swap(&mut i, &mut j)");
    println!("  Dereference: *a = *b");
    println!();
    println!("Key differences:");
    println!("• Rust references are always valid");
    println!("• No null references in Rust");
    println!("• Borrowing rules prevent data races");
}

