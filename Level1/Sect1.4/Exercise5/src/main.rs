// Exercise 5 - Section 1.4 Requirements:
// ---------------------------------------
// Create a C-program that prints two columns on the screen with the
// temperature in degrees Celsius in the left column and degrees Fahrenheit
// in the right column.
// Start with 0 degrees Celsius and proceed until 19 degrees Celsius.
// Take steps of 1 degree. Print degrees Fahrenheit with 1 position after
// the comma. Also print a header text.
// The result must be obtained using a for construction!
//
// Note: Formula to convert Celsius to Fahrenheit:
// Fahrenheit = (9/5) * Celsius + 32

fn main() {
    // Configuration constants
    const START_CELSIUS: i32 = 0;
    const END_CELSIUS: i32 = 19;
    const STEP_SIZE: i32 = 1;
    
    println!("╔════════════════════════════════════════╗");
    println!("║   CELSIUS TO FAHRENHEIT CONVERSION    ║");
    println!("╚════════════════════════════════════════╝");
    println!();
    
    // Print header
    println!("┌──────────────┬──────────────┐");
    println!("│    Celsius   │  Fahrenheit  │");
    println!("├──────────────┼──────────────┤");
    
    // For loop construction as required
    // Rust's for loop with range is idiomatic
    for celsius in (START_CELSIUS..=END_CELSIUS).step_by(STEP_SIZE as usize) {
        // Calculate Fahrenheit using the formula
        // Fahrenheit = (9/5) * Celsius + 32
        let fahrenheit = (9.0 / 5.0) * (celsius as f64) + 32.0;
        
        // Print the row with proper formatting
        // {:7} for celsius (integer), {:7.1} for fahrenheit (1 decimal)
        println!("│      {:2}      │     {:5.1}    │", celsius, fahrenheit);
    }
    
    println!("└──────────────┴──────────────┘");
    
    // Display the formula
    println!("\n╔════════════════════════════════════════╗");
    println!("║              FORMULA USED              ║");
    println!("╚════════════════════════════════════════╝");
    println!("  Fahrenheit = (9/5) × Celsius + 32");
    println!();
    println!("  Configuration:");
    println!("  • Start: {}°C", START_CELSIUS);
    println!("  • End:   {}°C", END_CELSIUS);
    println!("  • Step:  {}°C", STEP_SIZE);
    
    // Show important conversions in this range
    println!("\n╔════════════════════════════════════════╗");
    println!("║     NOTABLE TEMPERATURES IN RANGE      ║");
    println!("╚════════════════════════════════════════╝");
    println!("  •  0°C = 32.0°F (Water freezes)");
    println!("  • 10°C = 50.0°F (Cool day)");
    println!("  • 15°C = 59.0°F (Mild temperature)");
    println!("  • 19°C = 66.2°F (Room temperature)");
    
    // Alternative for loop styles in Rust
    println!("\n╔════════════════════════════════════════╗");
    println!("║        FOR LOOP VARIATIONS             ║");
    println!("╚════════════════════════════════════════╝");
    println!("  Rust for loop styles:");
    println!("  1. Range: for i in 0..20");
    println!("  2. Inclusive: for i in 0..=19");
    println!("  3. With step: (0..=19).step_by(1)");
    println!("  4. C-style simulation:");
    println!("     let mut i = 0;");
    println!("     while i <= 19 {{ ... i += 1; }}");
}

// Alternative implementation showing C-style for loop simulation
fn _c_style_for_loop_example() {
    println!("C-style for loop simulation in Rust:");
    
    // C: for(int celsius = 0; celsius <= 19; celsius++)
    // Rust equivalent using while:
    let mut celsius = 0;
    while celsius <= 19 {
        let fahrenheit = (9.0 / 5.0) * (celsius as f64) + 32.0;
        println!("{:2}°C = {:5.1}°F", celsius, fahrenheit);
        celsius += 1;
    }
}

// Another alternative using iterator
fn _iterator_style_example() {
    println!("Iterator style in Rust:");
    
    (0..=19).for_each(|celsius| {
        let fahrenheit = (9.0 / 5.0) * (celsius as f64) + 32.0;
        println!("{:2}°C = {:5.1}°F", celsius, fahrenheit);
    });
}
