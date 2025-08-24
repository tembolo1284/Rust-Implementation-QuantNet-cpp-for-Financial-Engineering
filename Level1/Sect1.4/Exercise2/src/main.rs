// Exercise 2 - Section 1.4 Requirements:
// ---------------------------------------
// Rewrite the C-program that was written for exercise 1,
// but use do-while instead of while.
//
// Note: Rust doesn't have do-while, but we can simulate it with
// loop { ... if condition { break; } }

use std::io::{self, Read};

fn main() {
    println!("Text Statistics Counter (do-while version)");
    println!("==========================================");
    println!("Type your text (press Ctrl+D on Unix/Mac or Ctrl+Z+Enter on Windows to finish):");
    println!();
    
    // Initialize counters
    let mut char_count = 0;
    let mut word_count = 0;
    let mut line_count = 0;
    let mut in_word = false;
    
    // Get stdin handle for byte-by-byte reading
    let stdin = io::stdin();
    let mut bytes = stdin.lock().bytes();
    
    // Flag to track if we should continue
    let mut continue_reading = true;
    
    // Rust equivalent of do-while: loop with condition check at the end
    loop {
        // Do block - execute at least once
        let byte_result = bytes.next();
        
        match byte_result {
            None => {
                // EOF reached
                continue_reading = false;
            }
            Some(Ok(byte)) => {
                let ch = byte as char;
                
                // Count every character
                char_count += 1;
                
                // Count lines
                if ch == '\n' {
                    line_count += 1;
                }
                
                // Word counting logic
                let is_whitespace = ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r';
                
                if is_whitespace {
                    if in_word {
                        in_word = false;
                    }
                } else {
                    if !in_word {
                        word_count += 1;
                        in_word = true;
                    }
                }
            }
            Some(Err(_)) => {
                eprintln!("Error reading input");
                continue_reading = false;
            }
        }
        
        // While condition check (at the end, like do-while)
        if !continue_reading {
            break;
        }
    }
    
    // Adjust line count if needed
    if char_count > 0 && line_count == 0 {
        line_count = 1;
    }
    
    // Display results
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║                    STATISTICS                      ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("  Characters: {:6}", char_count);
    println!("  Words:      {:6}", word_count);
    println!("  Lines:      {:6}", line_count);
    
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║              DO-WHILE EXPLANATION                  ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("C's do-while syntax:");
    println!("  do {{");
    println!("      // body executes at least once");
    println!("  }} while (condition);");
    println!();
    println!("Rust equivalent:");
    println!("  loop {{");
    println!("      // body executes at least once");
    println!("      if !condition {{ break; }}");
    println!("  }}");
    println!();
    println!("Key difference from while:");
    println!("• do-while: Check condition AFTER first iteration");
    println!("• while: Check condition BEFORE first iteration");
}

