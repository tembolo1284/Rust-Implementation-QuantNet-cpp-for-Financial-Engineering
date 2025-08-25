// Exercise 1 - Section 1.6 Requirements:
// ---------------------------------------
// Write a C-program that contains two print macro calls.
// The first prints the variable a, the second prints the variables a and b.
// Printing happens by the use of the PRINT1 and PRINT2 macros that accept arguments.
// These macros must be defined in an include-file.
// The variables a and b gets their value in the function main().
// Name the program "Macro.c" and the include-file "Defs.h".
// Don't forget to implement the mechanism to avoid multiple inclusion of the header file.
//
// Note: Rust uses macros differently than C. We'll demonstrate both Rust macros
// and show the C preprocessor equivalent.

// Include our macro definitions (similar to #include "defs.h")
#[macro_use]
mod defs;

use std::io;

fn main() {
    println!("Macro Demonstration Program");
    println!("===========================\n");
    
    // Get value for variable a
    println!("Enter value for variable 'a':");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let a: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    // Get value for variable b
    println!("Enter value for variable 'b':");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let b: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║            MACRO OUTPUTS               ║");
    println!("╚════════════════════════════════════════╝");
    
    // Use PRINT1 macro to print variable a
    println!("Using PRINT1 macro:");
    print1!(a);
    
    // Use PRINT2 macro to print variables a and b
    println!("\nUsing PRINT2 macro:");
    print2!(a, b);
    
    // Additional examples with different values
    println!("\n╔════════════════════════════════════════╗");
    println!("║          MORE EXAMPLES                 ║");
    println!("╚════════════════════════════════════════╝");
    
    let x = 42;
    let y = 17;
    
    println!("With x={} and y={}:", x, y);
    print!("  PRINT1(x): ");
    print1!(x);
    print!("  PRINT2(x, y): ");
    print2!(x, y);
    
    // Demonstrate expressions in macros
    println!("\n╔════════════════════════════════════════╗");
    println!("║      EXPRESSIONS IN MACROS             ║");
    println!("╚════════════════════════════════════════╝");
    
    println!("Macros can handle expressions:");
    print!("  PRINT1(a + b): ");
    print1!(a + b);
    print!("  PRINT2(a * 2, b - 1): ");
    print2!(a * 2, b - 1);
    
    // Explanation of preprocessor concepts
    println!("\n╔════════════════════════════════════════╗");
    println!("║     C PREPROCESSOR VS RUST MACROS      ║");
    println!("╚════════════════════════════════════════╝");
    println!("C Preprocessor:");
    println!("  • Text substitution before compilation");
    println!("  • #define PRINT1(x) printf(\"...\", x)");
    println!("  • Include guards prevent multiple inclusion");
    println!("  • No type checking at macro level");
    println!();
    println!("Rust Macros:");
    println!("  • Pattern matching and code generation");
    println!("  • macro_rules! print1 {{ ... }}");
    println!("  • Module system prevents duplication");
    println!("  • Hygienic and type-safe");
}
