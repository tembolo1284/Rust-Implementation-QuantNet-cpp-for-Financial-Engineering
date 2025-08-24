// Exercise 3 - Section 1.4 Requirements:
// ---------------------------------------
// Do exercise 1 again, but change your solution so that
// the switch-case statement is used instead of if blocks.
//
// Note: Rust uses match expressions instead of switch-case

use std::io::{self, Read};

fn main() {
    println!("Text Statistics Counter (switch-case/match version)");
    println!("===================================================");
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
    
    // While loop to read characters until EOF
    loop {
        let byte_result = bytes.next();
        
        match byte_result {
            None => break,  // EOF reached
            Some(Ok(byte)) => {
                let ch = byte as char;
                
                // Count every character
                char_count += 1;
                
                // Use match (Rust's switch-case) for character classification
                match ch {
                    '\n' => {
                        // Newline character
                        line_count += 1;
                        if in_word {
                            in_word = false;
                        }
                    }
                    ' ' | '\t' | '\r' => {
                        // Other whitespace characters
                        if in_word {
                            in_word = false;
                        }
                    }
                    _ => {
                        // Any other character (non-whitespace)
                        if !in_word {
                            word_count += 1;
                            in_word = true;
                        }
                    }
                }
            }
            Some(Err(_)) => {
                eprintln!("Error reading input");
                break;
            }
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
    println!("║            SWITCH-CASE vs MATCH                    ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("C's switch-case:");
    println!("  switch(ch) {{");
    println!("      case ' ':");
    println!("      case '\\t':");
    println!("      case '\\n':");
    println!("      case '\\r':");
    println!("          // handle whitespace");
    println!("          break;");
    println!("      default:");
    println!("          // handle non-whitespace");
    println!("          break;");
    println!("  }}");
    println!();
    println!("Rust's match:");
    println!("  match ch {{");
    println!("      ' ' | '\\t' | '\\n' | '\\r' => {{");
    println!("          // handle whitespace");
    println!("      }}");
    println!("      _ => {{");
    println!("          // handle non-whitespace");
    println!("      }}");
    println!("  }}");
    
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║              ADVANTAGES OF MATCH                   ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("• No fall-through bugs (no need for 'break')");
    println!("• Pattern matching with | for multiple cases");
    println!("• Compiler ensures all cases are handled");
    println!("• Can destructure and bind values in patterns");
    println!("• Returns a value (it's an expression)");
}

// Alternative implementation using more detailed match patterns
fn _alternative_char_classifier(ch: char, in_word: &mut bool, word_count: &mut i32) {
    // This shows a more elaborate match pattern
    match ch {
        // Specific whitespace characters
        ' ' => {
            if *in_word {
                *in_word = false;
            }
        }
        '\t' => {
            if *in_word {
                *in_word = false;
            }
        }
        '\n' | '\r' => {
            if *in_word {
                *in_word = false;
            }
        }
        // Ranges and patterns
        'a'..='z' | 'A'..='Z' | '0'..='9' => {
            if !*in_word {
                *word_count += 1;
                *in_word = true;
            }
        }
        // Special characters that count as word characters
        '_' | '-' | '\'' => {
            if !*in_word {
                *word_count += 1;
                *in_word = true;
            }
        }
        // Everything else
        _ => {
            if !*in_word {
                *word_count += 1;
                *in_word = true;
            }
        }
    }
}
