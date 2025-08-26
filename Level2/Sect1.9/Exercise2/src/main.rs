// Exercise 2 - Section 1.9 Requirements:
// ---------------------------------------
// Alter the last program of exercise 1 in such a way that the output
// doesn't go to the screen but is written to a file. The file to write
// to must be specified by the user.
//
// Note: This extends Exercise 1 to write to a file instead of stdout

use std::fs::File;
use std::io::{self, Read, Write, BufWriter};

const CTRL_A: u8 = 1;  // ASCII value of Ctrl+A

fn main() {
    println!("Character Echo to File Program");
    println!("==============================");
    
    // Get filename from user
    println!("Enter the output filename:");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read filename");
    let filename = filename.trim();
    
    // Create or open the file for writing
    let file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creating file '{}': {}", filename, e);
            return;
        }
    };
    
    // Use BufWriter for efficient file writing
    let mut file_writer = BufWriter::new(file);
    
    println!("\nFile '{}' opened for writing.", filename);
    println!("Type text and press Enter to save it to the file.");
    println!("Press Ctrl+A to exit and save the file.\n");
    
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
                println!("\nEOF reached. Saving file...");
                break;
            }
            Ok(_) => {
                let ch = byte_buffer[0];
                char_count += 1;
                
                // Check for Ctrl+A
                if ch == CTRL_A {
                    println!("\nCTRL + A is a correct ending.");
                    
                    // Write any remaining buffer content
                    if !line_buffer.is_empty() {
                        for &byte in &line_buffer {
                            file_writer.write(&[byte]).unwrap();
                        }
                        file_writer.write(b"\n").unwrap();
                        line_count += 1;
                    }
                    break;
                }
                
                // Check for Enter key (newline)
                if ch == b'\n' {
                    line_count += 1;
                    
                    // Write line to file
                    for &byte in &line_buffer {
                        file_writer.write(&[byte]).unwrap();
                    }
                    file_writer.write(b"\n").unwrap();
                    
                    // Echo to screen for confirmation
                    print!("Line {} written: ", line_count);
                    for &byte in &line_buffer {
                        print!("{}", byte as char);
                    }
                    println!();
                    stdout.flush().unwrap();
                    
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
    
    // Flush the file buffer to ensure all data is written
    file_writer.flush().unwrap();
    
    // Print statistics
    println!("\n╔════════════════════════════════════════╗");
    println!("║            FILE STATISTICS             ║");
    println!("╚════════════════════════════════════════╝");
    println!("Filename:              {}", filename);
    println!("Total characters read: {}", char_count);
    println!("Total lines written:   {}", line_count);
    
    // Read and display the file contents
    println!("\n╔════════════════════════════════════════╗");
    println!("║         FILE CONTENTS                  ║");
    println!("╚════════════════════════════════════════╝");
    
    match std::fs::read_to_string(filename) {
        Ok(contents) => {
            if contents.is_empty() {
                println!("(File is empty)");
            } else {
                println!("{}", contents);
            }
        }
        Err(e) => {
            eprintln!("Error reading file back: {}", e);
        }
    }
    
    // Show file operations comparison
    demonstrate_file_operations();
}

fn demonstrate_file_operations() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║      FILE OPERATIONS COMPARISON        ║");
    println!("╚════════════════════════════════════════╝");
    
    println!("C File Operations:");
    println!("  FILE *fp = fopen(\"file.txt\", \"w\");");
    println!("  if (fp == NULL) {{ /* error */ }}");
    println!("  int c = getchar();");
    println!("  fputc(c, fp);  // or fprintf(fp, \"%c\", c);");
    println!("  fclose(fp);");
    println!();
    
    println!("Rust File Operations:");
    println!("  let file = File::create(\"file.txt\")?;");
    println!("  let mut writer = BufWriter::new(file);");
    println!("  let mut buf = [0u8; 1];");
    println!("  stdin.read(&mut buf)?;");
    println!("  writer.write(&[buf[0]])?;");
    println!("  writer.flush()?;");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║         FILE MODES                     ║");
    println!("╚════════════════════════════════════════╝");
    println!("┌──────┬──────────────────────────────────┐");
    println!("│ Mode │ Description                      │");
    println!("├──────┼──────────────────────────────────┤");
    println!("│ \"r\"  │ Read (file must exist)           │");
    println!("│ \"w\"  │ Write (creates/truncates file)   │");
    println!("│ \"a\"  │ Append (creates if not exists)   │");
    println!("│ \"r+\" │ Read/Write (file must exist)     │");
    println!("│ \"w+\" │ Read/Write (creates/truncates)   │");
    println!("│ \"a+\" │ Read/Append                      │");
    println!("└──────┴──────────────────────────────────┘");
}

