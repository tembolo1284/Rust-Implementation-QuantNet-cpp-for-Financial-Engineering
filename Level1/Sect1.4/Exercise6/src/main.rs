// Exercise 6 - Section 1.4 Requirements:
// ---------------------------------------
// Create a C-program that counts how many times each of the numbers 0-4
// have been typed. Use a switch-case construction. Use default to count
// the number of other characters. The input will be halted with ^Z (EOF).
// EOF means End-of-File and is defined in <stdio.h>. Thus, the constant
// EOF can be used in a condition (test if EOF has been typed).
// Print the amount of times a certain number has been typed.
// Name the program freq.c
//
// Note: In Rust, we use match instead of switch-case

use std::io::{self, Read};

fn main() {
    println!("Character Frequency Counter");
    println!("============================");
    println!("Type characters (0-4 will be counted specially)");
    println!("Press Ctrl+D (Unix/Mac) or Ctrl+Z (Windows) to finish:");
    println!();
    
    // Initialize counters for digits 0-4 and other characters
    let mut count_0 = 0;
    let mut count_1 = 0;
    let mut count_2 = 0;
    let mut count_3 = 0;
    let mut count_4 = 0;
    let mut count_other = 0;
    
    // Get stdin handle for byte-by-byte reading
    let stdin = io::stdin();
    let mut bytes = stdin.lock().bytes();
    
    // Read characters until EOF
    loop {
        let byte_result = bytes.next();
        
        match byte_result {
            None => break,  // EOF reached (Ctrl+D or Ctrl+Z)
            Some(Ok(byte)) => {
                let ch = byte as char;
                
                // Switch-case construction (match in Rust)
                match ch {
                    '0' => count_0 += 1,
                    '1' => count_1 += 1,
                    '2' => count_2 += 1,
                    '3' => count_3 += 1,
                    '4' => count_4 += 1,
                    _ => count_other += 1,  // default case
                }
            }
            Some(Err(_)) => {
                eprintln!("Error reading input");
                break;
            }
        }
    }
    
    // Print the results
    println!("\n╔════════════════════════════════════════╗");
    println!("║         FREQUENCY RESULTS              ║");
    println!("╚════════════════════════════════════════╝");
    println!();
    println!("┌─────────────┬──────────┐");
    println!("│  Character  │  Count   │");
    println!("├─────────────┼──────────┤");
    println!("│      0      │    {:3}   │", count_0);
    println!("│      1      │    {:3}   │", count_1);
    println!("│      2      │    {:3}   │", count_2);
    println!("│      3      │    {:3}   │", count_3);
    println!("│      4      │    {:3}   │", count_4);
    println!("│    Others   │    {:3}   │", count_other);
    println!("└─────────────┴──────────┘");
    
    // Calculate total characters processed
    let total = count_0 + count_1 + count_2 + count_3 + count_4 + count_other;
    println!("\nTotal characters processed: {}", total);
    
    // Additional statistics
    println!("\n╔════════════════════════════════════════╗");
    println!("║          DETAILED ANALYSIS             ║");
    println!("╚════════════════════════════════════════╝");
    
    let digit_total = count_0 + count_1 + count_2 + count_3 + count_4;
    println!("Total digits (0-4): {}", digit_total);
    println!("Other characters:   {}", count_other);
    
    if total > 0 {
        let digit_percentage = (digit_total as f64 / total as f64) * 100.0;
        println!("Percentage of digits 0-4: {:.1}%", digit_percentage);
    }
    
    // Show which digit appeared most
    let max_count = count_0.max(count_1).max(count_2).max(count_3).max(count_4);
    if max_count > 0 {
        print!("\nMost frequent digit(s): ");
        if count_0 == max_count { print!("0 "); }
        if count_1 == max_count { print!("1 "); }
        if count_2 == max_count { print!("2 "); }
        if count_3 == max_count { print!("3 "); }
        if count_4 == max_count { print!("4 "); }
        println!("(appeared {} times)", max_count);
    }
}
