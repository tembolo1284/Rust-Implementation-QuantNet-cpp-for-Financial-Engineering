// Exercise 2 - Section 1.7 Requirements:
// ---------------------------------------
// The following program reads a string with a 30 character maximum.
// Implement the Length() function. The function Length() must determine
// the length of the string. Give Length() the address of the array as argument.
// Note: your Length() function should be similar to the built-in strlen()
// function so your job is to mimic that function without using it.
// EOF is used in the function main(). This means End-of-File.
// In DOS, EOF can be entered by the key combination Ctrl-z (often written as ^Z).
//
// Note: In Rust, we'll implement this using slices and references

use std::io::{self, Read};

const MAXLINE: usize = 30;

// Length function that mimics strlen() - takes a string slice
fn length(str: &[u8]) -> usize {
    let mut len = 0;
    
    // Count characters until we hit null terminator or end of slice
    for &ch in str {
        if ch == 0 {  // Null terminator found
            break;
        }
        len += 1;
    }
    
    len
}

// Alternative implementation using iterator
fn length_iter(str: &[u8]) -> usize {
    str.iter()
        .take_while(|&&ch| ch != 0)
        .count()
}

fn main() {
    println!("String Length Calculator");
    println!("========================");
    println!("Type up to {} chars. Exit with Ctrl+D (Unix/Mac) or Ctrl+Z (Windows)", MAXLINE);
    println!();
    
    // Create a buffer to store the string
    let mut string = [0u8; MAXLINE + 1];  // +1 for null terminator
    let mut i = 0;
    
    // Get stdin handle for byte-by-byte reading
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = [0u8; 1];
    
    // Read characters until EOF or MAXLINE reached
    while i < MAXLINE {
        match handle.read(&mut buffer) {
            Ok(0) => break,  // EOF reached
            Ok(_) => {
                let c = buffer[0];
                
                // Stop at newline or EOF
                if c == b'\n' {
                    break;
                }
                
                // Append character to string
                string[i] = c;
                i += 1;
            }
            Err(_) => break,
        }
    }
    
    // Null-terminate the string (C-style)
    string[i] = 0;
    
    // Calculate and print length
    let len = length(&string);
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║              RESULTS                   ║");
    println!("╚════════════════════════════════════════╝");
    
    // Convert to Rust string for display (up to null terminator)
    let display_string = String::from_utf8_lossy(&string[..i]);
    println!("String entered: \"{}\"", display_string);
    println!("String length is {} (using Length function)", len);
    
    // Verify with alternative implementation
    let len_iter = length_iter(&string);
    println!("String length is {} (using iterator version)", len_iter);
    
    // Also show Rust's native way
    println!("String length is {} (using Rust's .len())", display_string.len());
    
    // Test with various strings
    println!("\n╔════════════════════════════════════════╗");
    println!("║            TEST CASES                  ║");
    println!("╚════════════════════════════════════════╝");
    
    // Test different strings
    let test_cases: [&[u8]; 5] = [
        b"Hello\0",
        b"Test\0",
        b"A longer string here\0",
        b"\0",  // Empty string
        b"12345\0",
    ];
    
    for test in &test_cases {
        let len = length(test);
        let s = String::from_utf8_lossy(test);
        let s = s.trim_matches('\0');
        println!("Length of \"{}\" = {}", s, len);
    }
    
    // Show how it works
    println!("\n╔════════════════════════════════════════╗");
    println!("║         HOW LENGTH() WORKS             ║");
    println!("╚════════════════════════════════════════╝");
    println!("1. Start at the beginning of the string");
    println!("2. Count each character");
    println!("3. Stop when null terminator (\\0) is found");
    println!("4. Return the count");
    println!();
    println!("Example: \"Hello\\0\"");
    println!("  H(1) e(2) l(3) l(4) o(5) \\0(stop)");
    println!("  Length = 5");
    
    // C vs Rust comparison
    println!("\n╔════════════════════════════════════════╗");
    println!("║      C STRINGS VS RUST STRINGS         ║");
    println!("╚════════════════════════════════════════╝");
    println!("C Strings:");
    println!("• Null-terminated (\\0 at the end)");
    println!("• No length stored");
    println!("• strlen() counts until \\0");
    println!("• Can have buffer overflows");
    println!();
    println!("Rust Strings:");
    println!("• Length stored with string");
    println!("• No null terminator needed");
    println!("• .len() returns stored length");
    println!("• Memory safe, no overflows");
    
    // Implementation details
    println!("\n╔════════════════════════════════════════╗");
    println!("║     IMPLEMENTATION COMPARISON          ║");
    println!("╚════════════════════════════════════════╝");
    println!("C strlen implementation:");
    println!("  int length(char *str) {{");
    println!("      int len = 0;");
    println!("      while (*str != '\\0') {{");
    println!("          len++;");
    println!("          str++;");
    println!("      }}");
    println!("      return len;");
    println!("  }}");
    println!();
    println!("Our Rust implementation:");
    println!("  fn length(str: &[u8]) -> usize {{");
    println!("      let mut len = 0;");
    println!("      for &ch in str {{");
    println!("          if ch == 0 {{ break; }}");
    println!("          len += 1;");
    println!("      }}");
    println!("      len");
    println!("  }}");
}

