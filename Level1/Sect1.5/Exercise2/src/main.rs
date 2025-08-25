// Exercise 2 - Section 1.5 Requirements:
// ---------------------------------------
// Write a C-program that prints the factorials of a number.
// 6! (six factorial) is the same as 6 * 5 * 4 * 3 * 2 * 1
// Must make use of a recursive function.

use std::io;

// Recursive factorial function
fn factorial(n: u32) -> u64 {
    // Base case: 0! = 1 and 1! = 1
    if n <= 1 {
        1
    } else {
        // Recursive case: n! = n * (n-1)!
        n as u64 * factorial(n - 1)
    }
}

// Helper function to show the calculation steps
fn factorial_with_steps(n: u32, depth: usize) -> u64 {
    // Print indentation to show recursion depth
    for _ in 0..depth {
        print!("  ");
    }
    
    if n <= 1 {
        println!("factorial({}) = 1 (base case)", n);
        1
    } else {
        println!("factorial({}) = {} × factorial({})", n, n, n - 1);
        let result = n as u64 * factorial_with_steps(n - 1, depth + 1);
        
        // Print the return value at this level
        for _ in 0..depth {
            print!("  ");
        }
        println!("factorial({}) returns {}", n, result);
        result
    }
}

fn main() {
    println!("Recursive Factorial Calculator");
    println!("==============================");
    
    // Get number from user
    println!("Enter a non-negative integer:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let number: u32 = input.trim().parse()
        .expect("Please enter a valid non-negative integer");
    
    // Check for overflow risk
    if number > 20 {
        println!("Warning: Factorials larger than 20! may overflow!");
    }
    
    // Calculate factorial
    let result = factorial(number);
    
    // Print result
    println!("\n╔════════════════════════════════════════╗");
    println!("║                RESULT                  ║");
    println!("╚════════════════════════════════════════╝");
    println!("{}! = {}", number, result);
    
    // Show the expansion
    if number > 0 {
        print!("{}! = ", number);
        for i in (1..=number).rev() {
            print!("{}", i);
            if i > 1 {
                print!(" × ");
            }
        }
        println!(" = {}", result);
    }
    
    // Show recursion trace for small numbers
    if number <= 6 {
        println!("\n╔════════════════════════════════════════╗");
        println!("║           RECURSION TRACE              ║");
        println!("╚════════════════════════════════════════╝");
        factorial_with_steps(number, 0);
    }
    
    // Show example factorials
    println!("\n╔════════════════════════════════════════╗");
    println!("║         FACTORIAL TABLE                ║");
    println!("╚════════════════════════════════════════╝");
    println!("┌──────┬─────────────────┐");
    println!("│  n   │       n!        │");
    println!("├──────┼─────────────────┤");
    for i in 0..=10 {
        println!("│  {:2}  │ {:15} │", i, factorial(i));
    }
    println!("└──────┴─────────────────┘");
    
    // Explanation of recursion
    println!("\n╔════════════════════════════════════════╗");
    println!("║      HOW RECURSION WORKS               ║");
    println!("╚════════════════════════════════════════╝");
    println!("• Base case: 0! = 1 and 1! = 1");
    println!("• Recursive case: n! = n × (n-1)!");
    println!("• Each call waits for the next to return");
    println!("• Results bubble up from base case");
}
