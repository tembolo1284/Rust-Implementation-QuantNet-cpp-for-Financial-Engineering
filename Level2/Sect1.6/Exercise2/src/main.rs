// Exercise 2 - Section 1.6 Requirements:
// ---------------------------------------
// Create the two macros MAX2(x,y) and MAX3(x,y,z).
// These macros must return the maximum value of the given arguments.
// Let the macro MAX3 make use of the macro MAX2.
// Add these macros to the file "Defs.h".

// Include our macro definitions
#[macro_use]
mod defs;

use std::io;

fn main() {
    println!("Maximum Value Macros Demonstration");
    println!("===================================\n");
    
    // Get three values from user
    println!("Enter first value (x):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let x: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    println!("Enter second value (y):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let y: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    println!("Enter third value (z):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let z: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║            MACRO RESULTS               ║");
    println!("╚════════════════════════════════════════╝");
    
    // Use MAX2 macro
    let max_xy = max2!(x, y);
    println!("MAX2({}, {}) = {}", x, y, max_xy);
    
    // Use MAX3 macro (which internally uses MAX2)
    let max_xyz = max3!(x, y, z);
    println!("MAX3({}, {}, {}) = {}", x, y, z, max_xyz);
    
    // Additional test cases
    println!("\n╔════════════════════════════════════════╗");
    println!("║          TEST CASES                    ║");
    println!("╚════════════════════════════════════════╝");
    
    // Test with different combinations
    println!("Testing with various values:");
    println!("  MAX2(5, 3) = {}", max2!(5, 3));
    println!("  MAX2(-2, -5) = {}", max2!(-2, -5));
    println!("  MAX2(0, 0) = {}", max2!(0, 0));
    println!();
    println!("  MAX3(1, 2, 3) = {}", max3!(1, 2, 3));
    println!("  MAX3(3, 2, 1) = {}", max3!(3, 2, 1));
    println!("  MAX3(2, 3, 1) = {}", max3!(2, 3, 1));
    println!("  MAX3(-1, -2, -3) = {}", max3!(-1, -2, -3));
    println!("  MAX3(5, 5, 5) = {}", max3!(5, 5, 5));
    
    // Demonstrate with expressions
    println!("\n╔════════════════════════════════════════╗");
    println!("║      MACROS WITH EXPRESSIONS           ║");
    println!("╚════════════════════════════════════════╝");
    
    println!("Macros work with expressions too:");
    println!("  MAX2(x+1, y*2) = {}", max2!(x+1, y*2));
    println!("  MAX3(x-1, y+1, z*2) = {}", max3!(x-1, y+1, z*2));
    
    // Show how MAX3 uses MAX2
    println!("\n╔════════════════════════════════════════╗");
    println!("║     HOW MAX3 USES MAX2                 ║");
    println!("╚════════════════════════════════════════╝");
    println!("MAX3(x, y, z) expands to:");
    println!("  MAX2(MAX2(x, y), z)");
    println!();
    println!("Example: MAX3({}, {}, {})", x, y, z);
    println!("  Step 1: MAX2({}, {}) = {}", x, y, max2!(x, y));
    println!("  Step 2: MAX2({}, {}) = {}", max2!(x, y), z, max3!(x, y, z));
    
    // Preprocessor pitfalls
    println!("\n╔════════════════════════════════════════╗");
    println!("║    C PREPROCESSOR PITFALLS             ║");
    println!("╚════════════════════════════════════════╝");
    println!("Common issues with C macros:");
    println!("1. Side effects: MAX2(i++, j++) increments twice!");
    println!("2. Operator precedence: Need parentheses");
    println!("3. Type safety: No type checking");
    println!("4. Debugging: Hard to debug macro expansions");
    println!();
    println!("Rust macros are safer:");
    println!("• Hygienic (no name collisions)");
    println!("• Type-checked after expansion");
    println!("• Better error messages");
}

