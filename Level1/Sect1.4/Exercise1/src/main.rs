// Exercise 1 - Section 1.4 Requirements:
// ---------------------------------------
// Write a C-program that asks for text input from the keyboard.
// The output of this program should be the amount of characters,
// the amount of words and the amount of lines that have been typed.
// Multiple consecutive spaces should NOT be counted as multiple words.
// Reading keys from the keyboard is possible by using the function getchar().
// The reading of characters from the keyboard can be stopped when the
// shutdown-code ^D (CTRL + D) is entered. ^D has the ASCII-value 4.
// Use a while loop.
//
// Note: In Rust, we read from stdin byte by byte to mimic getchar()

use std::io::{self, Read};

fn main() {
    println!("Text Statistics Counter");
    println!("=======================");
    println!("Type your text (press Ctrl+D on Unix/Mac or Ctrl+Z+Enter on Windows to finish):");
    println!();
    
    // Initialize counters
    let mut char_count = 0;
    let mut word_count = 0;
    let mut line_count = 0;
    let mut in_word = false;  // Track if we're currently inside a word
    
    // Get stdin handle for byte-by-byte reading
    let stdin = io::stdin();
    let mut bytes = stdin.lock().bytes();
    
    // While loop to read characters until EOF (similar to C's getchar())
    loop {
        // Read next byte (similar to getchar())
        let byte_result = bytes.next();
        
        // Check for EOF (None means end of input)
        match byte_result {
            None => break,  // EOF reached (Ctrl+D on Unix, Ctrl+Z on Windows)
            Some(Ok(byte)) => {
                let ch = byte as char;
                
                // Count every character
                char_count += 1;
                
                // Count lines
                if ch == '\n' {
                    line_count += 1;
                }
                
                // Word counting logic
                // Check if current character is whitespace
                let is_whitespace = ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r';
                
                if is_whitespace {
                    // We're at whitespace
                    if in_word {
                        // We were in a word, now we're leaving it
                        in_word = false;
                    }
                    // Multiple consecutive spaces: do nothing (already outside word)
                } else {
                    // We're at a non-whitespace character
                    if !in_word {
                        // We weren't in a word, now entering one
                        word_count += 1;
                        in_word = true;
                    }
                    // If already in a word, just continue
                }
                
                // Optional: Show what's being typed (comment out for cleaner output)
                // print!("{}", ch);
                // io::Write::flush(&mut io::stdout()).unwrap();
            }
            Some(Err(_)) => {
                // Error reading input
                eprintln!("Error reading input");
                break;
            }
        }
    }
    
    // Adjust line count if file doesn't end with newline but has content
    if char_count > 0 && line_count == 0 {
        line_count = 1;  // At least one line if there's any content
    }
    
    // Display results
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║                    STATISTICS                      ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("  Characters: {:6}", char_count);
    println!("  Words:      {:6}", word_count);
    println!("  Lines:      {:6}", line_count);
    
    // Test examples
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║                  TEST EXAMPLES                     ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("Try these inputs to test:");
    println!("1. 'Hello World' → 11 chars, 2 words, 1 line");
    println!("2. 'Hello    World' (multiple spaces) → 14 chars, 2 words, 1 line");
    println!("3. 'Hello\\nWorld' (with newline) → 11 chars, 2 words, 2 lines");
    println!("4. '  Hello  ' (leading/trailing spaces) → 9 chars, 1 word, 1 line");
    
    // Algorithm explanation
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║              ALGORITHM EXPLANATION                 ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("The word counting algorithm uses a state machine:");
    println!("• 'in_word' flag tracks if we're inside a word");
    println!("• Entering a word (transition from space to non-space) increments counter");
    println!("• Multiple spaces don't create multiple words");
    println!("• This mimics the behavior of Unix 'wc' command");
}
