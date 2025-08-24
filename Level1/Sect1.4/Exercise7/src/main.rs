use std::io::{self, Read};

fn main() {
    println!("Character Frequency Counter (Extended)");
    println!("=======================================");
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
            None => break,  // EOF reached
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
    
    // Print the standard results
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
    
    // Special handling for number 3 - show frequency in words
    println!("\n╔════════════════════════════════════════╗");
    println!("║    SPECIAL REPORT FOR NUMBER THREE     ║");
    println!("╚════════════════════════════════════════╝");
    println!();
    
    // Convert count to words for number 3
    match count_3 {
        0 => println!("Number three appears zero times."),
        1 => println!("Number three appears one time."),
        2 => println!("Number three appears two times."),
        _ => println!("The number three appears more than two times."),
    }
    
    // Additional detailed information about '3'
    if count_3 >= 3 {
        println!("(Actual count: {} times)", count_3);
    }
    
    // Calculate total and show statistics
    let total = count_0 + count_1 + count_2 + count_3 + count_4 + count_other;
    println!("\n╔════════════════════════════════════════╗");
    println!("║          STATISTICAL SUMMARY           ║");
    println!("╚════════════════════════════════════════╝");
    println!("Total characters processed: {}", total);
    
    if total > 0 {
        let three_percentage = (count_3 as f64 / total as f64) * 100.0;
        println!("Percentage of '3' characters: {:.1}%", three_percentage);
    }
    
    // Helper function to convert number to word (for demonstration)
    fn number_to_word(n: i32) -> &'static str {
        match n {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            _ => "many",
        }
    }
    
    // Show all counts in words (bonus feature)
    println!("\n╔════════════════════════════════════════╗");
    println!("║         ALL COUNTS IN WORDS           ║");
    println!("╚════════════════════════════════════════╝");
    println!("Character '0': {} time(s)", number_to_word(count_0.min(5)));
    println!("Character '1': {} time(s)", number_to_word(count_1.min(5)));
    println!("Character '2': {} time(s)", number_to_word(count_2.min(5)));
    println!("Character '3': {} time(s)", number_to_word(count_3.min(5)));
    println!("Character '4': {} time(s)", number_to_word(count_4.min(5)));
}
