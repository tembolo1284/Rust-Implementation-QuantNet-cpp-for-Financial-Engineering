use std::io::{self, Read, Write};

const CTRL_A: u8 = 1;  // ASCII value of Ctrl+A

fn main() {
    println!("Character Echo Program");
    println!("======================");
    println!("Type text and press Enter to see it echoed.");
    println!("Press Ctrl+A to exit properly.\n");
    
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    // Buffer to store the current line
    let mut line_buffer: Vec<u8> = Vec::new();
    let mut char_count = 0;
    let mut line_count = 0;
    
    // Get stdin handle for byte-by-byte reading
    let mut handle = stdin.lock();
    let mut byte_buffer = [0u8; 1];
    
    // Main loop - read characters until Ctrl+A
    loop {
        // Read one character (like getchar())
        match handle.read(&mut byte_buffer) {
            Ok(0) => {
                // EOF reached
                println!("\nEOF reached. Program ending.");
                break;
            }
            Ok(_) => {
                let ch = byte_buffer[0];
                char_count += 1;
                
                // Check for Ctrl+A
                if ch == CTRL_A {
                    println!("\nCTRL + A is a correct ending.");
                    break;
                }
                
                // Check for Enter key (newline)
                if ch == b'\n' {
                    // Echo the complete line
                    line_count += 1;
                    print!("Line {}: ", line_count);
                    
                    // Print each character in the buffer (like putchar())
                    for &byte in &line_buffer {
                        print!("{}", byte as char);
                        stdout.flush().unwrap();
                    }
                    println!(); // Add newline after echoing
                    
                    // Clear the buffer for next line
                    line_buffer.clear();
                } else if ch != b'\r' {  // Ignore carriage return
                    // Add character to buffer
                    line_buffer.push(ch);
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
    
    // Print statistics
    println!("\n╔════════════════════════════════════════╗");
    println!("║            STATISTICS                  ║");
    println!("╚════════════════════════════════════════╝");
    println!("Total characters read: {}", char_count);
    println!("Total lines processed: {}", line_count);
}

// Alternative implementation using a more C-like approach
fn _c_style_implementation() {
    println!("C-Style Character Echo");
    println!("======================\n");
    
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer: Vec<u8> = Vec::new();
    
    loop {
        // Read one byte (like getchar())
        let mut input = [0u8; 1];
        if stdin.lock().read(&mut input).unwrap() == 0 {
            break;
        }
        
        let c = input[0];
        
        // Check for Ctrl+A
        if c == CTRL_A {
            println!("\nCTRL + A is a correct ending.");
            break;
        }
        
        // Store character
        buffer.push(c);
        
        // If newline, echo the line
        if c == b'\n' {
            // Echo each character (like putchar())
            for &ch in &buffer {
                write!(stdout, "{}", ch as char).unwrap();
            }
            stdout.flush().unwrap();
            buffer.clear();
        }
    }
}

