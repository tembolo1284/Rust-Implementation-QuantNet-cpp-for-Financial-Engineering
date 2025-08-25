// Exercise 4 - Section 1.5 Requirements:
// ---------------------------------------
// Write a recursive function printnumber() which gets the number to be printed.
// This number is an integer. The function should print the number digit by digit
// by using the putchar() function. Don't use printf().
// Tips: Use the modulo operator (%) to determine the digit to print.
// Use the division operator (/) to calculate the argument for the recursive call.
// Don't forget to handle negative numbers correctly.
//
// Note: In Rust, we'll use print! with single characters to simulate putchar()

use std::io::{self, Write};

// Recursive function to print a number digit by digit
fn printnumber(n: i32) {
    // Handle negative numbers
    if n < 0 {
        // Print minus sign
        print!("-");
        io::stdout().flush().unwrap();
        
        // Make positive and continue
        // Special case for i32::MIN to avoid overflow
        if n == i32::MIN {
            print!("2147483648");
            io::stdout().flush().unwrap();
            return;
        }
        printnumber(-n);
        return;
    }
    
    // Base case: single digit
    if n < 10 {
        // Convert digit to character and print
        let digit_char = (b'0' + n as u8) as char;
        print!("{}", digit_char);
        io::stdout().flush().unwrap();
    } else {
        // Recursive case: more than one digit
        // Print all digits except the last one
        printnumber(n / 10);
        
        // Print the last digit
        let last_digit = n % 10;
        let digit_char = (b'0' + last_digit as u8) as char;
        print!("{}", digit_char);
        io::stdout().flush().unwrap();
    }
}

// Helper function to show the recursion process
fn printnumber_with_trace(n: i32, depth: usize) {
    // Print indentation for visualization
    println!();
    for _ in 0..depth {
        print!("  ");
    }
    print!("printnumber({}) called", n);
    
    if n < 0 {
        println!(" - negative, print '-' and call printnumber({})", -n);
        print!("-");
        io::stdout().flush().unwrap();
        if n == i32::MIN {
            print!("2147483648");
            io::stdout().flush().unwrap();
            return;
        }
        printnumber_with_trace(-n, depth + 1);
        return;
    }
    
    if n < 10 {
        println!(" - base case, print '{}'", n);
        let digit_char = (b'0' + n as u8) as char;
        print!("{}", digit_char);
        io::stdout().flush().unwrap();
    } else {
        let last_digit = n % 10;
        let remaining = n / 10;
        println!(" - recursive, split into {} and {}", remaining, last_digit);
        
        printnumber_with_trace(remaining, depth + 1);
        
        println!();
        for _ in 0..depth {
            print!("  ");
        }
        println!("Returning from printnumber({}), print last digit '{}'", n, last_digit);
        
        let digit_char = (b'0' + last_digit as u8) as char;
        print!("{}", digit_char);
        io::stdout().flush().unwrap();
    }
}

fn main() {
    println!("Recursive Number Printer (putchar style)");
    println!("=========================================");
    
    // Get number from user
    println!("Enter an integer:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let number: i32 = input.trim().parse()
        .expect("Please enter a valid integer");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║              OUTPUT                    ║");
    println!("╚════════════════════════════════════════╝");
    print!("The number is: ");
    io::stdout().flush().unwrap();
    printnumber(number);
    println!();
    
    // Test various numbers
    println!("\n╔════════════════════════════════════════╗");
    println!("║            TEST CASES                  ║");
    println!("╚════════════════════════════════════════╝");
    
    let test_numbers = [0, 5, 42, 123, -789, 1000, -1];
    for &num in &test_numbers {
        print!("printnumber({:6}): ", num);
        io::stdout().flush().unwrap();
        printnumber(num);
        println!();
    }
    
    // Show recursion trace for a smaller number
    if number.abs() < 1000 && number != 0 {
        println!("\n╔════════════════════════════════════════╗");
        println!("║          RECURSION TRACE               ║");
        println!("╚════════════════════════════════════════╝");
        println!("Tracing printnumber({}):", number);
        printnumber_with_trace(number, 0);
        println!();
    }
    
    // Algorithm explanation
    println!("\n╔════════════════════════════════════════╗");
    println!("║         ALGORITHM EXPLANATION          ║");
    println!("╚════════════════════════════════════════╝");
    println!("1. If negative: print '-' and recurse with positive");
    println!("2. If < 10: print the single digit (base case)");
    println!("3. Otherwise:");
    println!("   • Recurse with n/10 (all digits except last)");
    println!("   • Print n%10 (the last digit)");
    println!("\nExample: printnumber(123)");
    println!("  → printnumber(12) then print '3'");
    println!("    → printnumber(1) then print '2'");
    println!("      → print '1' (base case)");
    println!("Result: '1' '2' '3'");
}
