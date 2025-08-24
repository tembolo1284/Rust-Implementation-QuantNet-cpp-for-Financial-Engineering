// Exercise 6 Requirements:
// ------------------------
// Write a C-program that shifts any number two places to the right.
// Input should be an integer. Output should be the shifted result,
// as well as output an indication of whether a logical or arithmetic shift
// is performed (if a 1 or 0 is shifted in at the left side) for the
// inputted number.
//
// Note: In Rust, >> operator performs arithmetic shift for signed integers
// and logical shift for unsigned integers.

use std::io;

fn main() {
    println!("Right Shift Demonstration (>> 2)");
    println!("=================================\n");
    
    println!("Enter an integer:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let number: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid integer");
    
    // Perform arithmetic shift on signed integer
    let arithmetic_result = number >> 2;
    
    // Convert to unsigned for logical shift
    let number_unsigned = number as u32;
    let logical_result = number_unsigned >> 2;
    
    // Display results
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║                    RESULTS                        ║");
    println!("╚════════════════════════════════════════════════════╝");
    
    println!("\nOriginal number: {}", number);
    println!("Binary representation: {:032b}", number);
    
    println!("\n--- ARITHMETIC SHIFT (signed) ---");
    println!("Result: {} >> 2 = {}", number, arithmetic_result);
    println!("Binary: {:032b}", arithmetic_result);
    
    // Determine what was shifted in
    if number < 0 {
        println!("Sign bit is 1, so 1s are shifted in from the left");
        println!("Type: ARITHMETIC SHIFT (sign extension)");
    } else {
        println!("Sign bit is 0, so 0s are shifted in from the left");
        println!("Type: ARITHMETIC SHIFT (but behaves like logical for positive)");
    }
    
    println!("\n--- LOGICAL SHIFT (unsigned) ---");
    println!("Result: {} >> 2 = {} (as unsigned)", number_unsigned, logical_result);
    println!("Binary: {:032b}", logical_result);
    println!("Type: LOGICAL SHIFT (always shifts in 0s)");
    
    // Show the difference clearly
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║               SHIFT TYPE ANALYSIS                 ║");
    println!("╚════════════════════════════════════════════════════╝");
    
    if number < 0 {
        println!("\nFor NEGATIVE number {}:", number);
        println!("• Arithmetic shift: {} (preserves sign)", arithmetic_result);
        println!("• Logical shift:    {} (treats as unsigned)", logical_result as i32);
        println!("\nDifference: Arithmetic preserves the sign bit!");
        
        // Show bit pattern comparison
        println!("\nBit patterns (focusing on leftmost bits):");
        println!("Original:    11... (negative)");
        println!("Arithmetic:  11... (still negative)");
        println!("Logical:     00... (becomes positive)");
    } else {
        println!("\nFor POSITIVE number {}:", number);
        println!("• Arithmetic shift: {} ", arithmetic_result);
        println!("• Logical shift:    {} ", logical_result);
        println!("\nNo difference: Both shift in 0s for positive numbers");
    }
    
    // Educational note
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║                    EXPLANATION                    ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("• ARITHMETIC SHIFT: Preserves the sign bit");
    println!("  - For negative numbers: shifts in 1s");
    println!("  - For positive numbers: shifts in 0s");
    println!("  - Used for: dividing by powers of 2");
    println!("\n• LOGICAL SHIFT: Always shifts in 0s");
    println!("  - Treats the number as a bit pattern");
    println!("  - No special treatment for sign bit");
    println!("  - Used for: bit manipulation");
    
    // Example with -8
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║                 EXAMPLE: -8 >> 2                  ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("Try entering -8 to see the difference!");
    println!("• Arithmetic: -8 >> 2 = -2 (division by 4)");
    println!("• Logical:    Would give huge positive number");
}
