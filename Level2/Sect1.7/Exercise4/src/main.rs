// Exercise 4 - Section 1.7 Requirements:
// ---------------------------------------
// Create a C-program that has a function DayName() which can print
// the day of a given day-number. For example:
// 1 gives: Day 1 is a Sunday
// 7 gives: Day 7 is a Saturday
// The day-name (1-7) should be written "hard-coded" into the program
// using an array of strings.
//
// Note: In Rust, we use an array of &str (string slices)

use std::io;

// Function to print the day name for a given day number
fn day_name(day_number: i32) {
    // Array of strings (hard-coded day names)
    // Index 0 is unused to make days 1-7 map directly
    let days: [&str; 8] = [
        "",           // Index 0 (unused)
        "Sunday",     // Index 1
        "Monday",     // Index 2
        "Tuesday",    // Index 3
        "Wednesday",  // Index 4
        "Thursday",   // Index 5
        "Friday",     // Index 6
        "Saturday",   // Index 7
    ];
    
    // Check if day number is valid
    if day_number >= 1 && day_number <= 7 {
        println!("Day {} is a {}", day_number, days[day_number as usize]);
    } else {
        println!("Error: Invalid day number {}. Please use 1-7.", day_number);
    }
}

// Alternative implementation using 0-based indexing
fn day_name_zero_based(day_number: i32) {
    // Array starting at index 0 for Sunday
    let days: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    
    if day_number >= 1 && day_number <= 7 {
        // Subtract 1 to convert 1-7 to 0-6 array indices
        println!("Day {} is a {}", day_number, days[(day_number - 1) as usize]);
    } else {
        println!("Error: Invalid day number {}. Please use 1-7.", day_number);
    }
}

// Implementation using match (Rust idiomatic way)
fn day_name_match(day_number: i32) {
    let day = match day_number {
        1 => "Sunday",
        2 => "Monday",
        3 => "Tuesday",
        4 => "Wednesday",
        5 => "Thursday",
        6 => "Friday",
        7 => "Saturday",
        _ => {
            println!("Error: Invalid day number {}. Please use 1-7.", day_number);
            return;
        }
    };
    
    println!("Day {} is a {}", day_number, day);
}

fn main() {
    println!("Day Name Printer");
    println!("================\n");
    
    // Get day number from user
    println!("Enter a day number (1-7):");
    println!("1 = Sunday, 2 = Monday, ..., 7 = Saturday");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let day_number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };
    
    // Call the day_name function
    println!("\n╔════════════════════════════════════════╗");
    println!("║              RESULT                    ║");
    println!("╚════════════════════════════════════════╝");
    day_name(day_number);
    
    // Test all days
    println!("\n╔════════════════════════════════════════╗");
    println!("║         ALL DAYS OF THE WEEK           ║");
    println!("╚════════════════════════════════════════╝");
    for i in 1..=7 {
        day_name(i);
    }
    
    // Test edge cases
    println!("\n╔════════════════════════════════════════╗");
    println!("║           EDGE CASES                   ║");
    println!("╚════════════════════════════════════════╝");
    day_name(0);   // Invalid
    day_name(8);   // Invalid
    day_name(-1);  // Invalid
    
    // Show different implementations
    println!("\n╔════════════════════════════════════════╗");
    println!("║      ALTERNATIVE IMPLEMENTATIONS       ║");
    println!("╚════════════════════════════════════════╝");
    
    println!("\nUsing zero-based array:");
    for i in 1..=7 {
        day_name_zero_based(i);
    }
    
    println!("\nUsing match statement:");
    for i in 1..=7 {
        day_name_match(i);
    }
    
    // Array of strings explanation
    println!("\n╔════════════════════════════════════════╗");
    println!("║      ARRAY OF STRINGS EXPLAINED        ║");
    println!("╚════════════════════════════════════════╝");
    println!("C version:");
    println!("  char *days[] = {{");
    println!("      \"\",          // Index 0 (unused)");
    println!("      \"Sunday\",    // Index 1");
    println!("      \"Monday\",    // Index 2");
    println!("      // ... etc");
    println!("  }};");
    println!();
    println!("Rust version:");
    println!("  let days: [&str; 8] = [");
    println!("      \"\",          // Index 0 (unused)");
    println!("      \"Sunday\",    // Index 1");
    println!("      \"Monday\",    // Index 2");
    println!("      // ... etc");
    println!("  ];");
    
    // Memory layout
    println!("\n╔════════════════════════════════════════╗");
    println!("║         MEMORY LAYOUT                  ║");
    println!("╚════════════════════════════════════════╝");
    println!("Array of strings in memory:");
    println!("┌─────────┬──────────────┐");
    println!("│ Index   │ String       │");
    println!("├─────────┼──────────────┤");
    
    let days: [&str; 8] = ["", "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    for i in 0..8 {
        if i == 0 {
            println!("│ [{}]     │ \"{}\" (unused) │", i, days[i]);
        } else {
            println!("│ [{}]     │ \"{:<10}\" │", i, days[i]);
        }
    }
    println!("└─────────┴──────────────┘");
    
    // Common patterns
    println!("\n╔════════════════════════════════════════╗");
    println!("║         COMMON PATTERNS                ║");
    println!("╚════════════════════════════════════════╝");
    println!("1. Direct indexing (waste index 0):");
    println!("   - Pro: Simple, direct mapping");
    println!("   - Con: Wastes one array element");
    println!();
    println!("2. Zero-based with offset:");
    println!("   - Pro: No wasted space");
    println!("   - Con: Need to subtract 1 from input");
    println!();
    println!("3. Match/Switch statement:");
    println!("   - Pro: Very clear and explicit");
    println!("   - Con: More verbose");
    
    // Calendar context
    println!("\n╔════════════════════════════════════════╗");
    println!("║         CALENDAR CONTEXT               ║");
    println!("╚════════════════════════════════════════╝");
    println!("Different week start conventions:");
    println!("• Sunday-start: Common in USA, Canada");
    println!("• Monday-start: ISO 8601, most of Europe");
    println!();
    println!("Our mapping (Sunday = 1):");
    for i in 1..=7 {
        let days = ["", "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        print!("{} = {} ", i, days[i]);
    }
    println!();
}

