// Exercise 4 - Section 1.4 Requirements:
// ---------------------------------------
// Create a C-program that prints two columns on the screen with the temperature
// in degrees Fahrenheit and the equivalent temperature in degrees Celsius.
// The left column shows the temperature in Fahrenheit.
// The right column shows the temperature in Celsius.
// Start with 0 degrees Fahrenheit and proceed until 300 degrees Fahrenheit.
// Take steps of 20 degrees. Print degrees Celsius with 1 position behind the comma
// (use "%10.1f" as format specifier). Also print a header text.
// Make the program maintenance insensitive, which means that the start- and
// end-temperature and the step size must be easy to adjust.
// The result must be obtained using a while construction!
// Tip: To calculate the degrees Celsius from degrees Fahrenheit use:
// Celsius = (5/9) * (Fahrenheit – 32)
//
// Note: In Rust, we use format specifiers like {:10.1} instead of %10.1f

fn main() {
    // Maintenance-friendly constants (easy to adjust)
    const START_FAHRENHEIT: f64 = 0.0;
    const END_FAHRENHEIT: f64 = 300.0;
    const STEP_SIZE: f64 = 20.0;
    
    println!("╔════════════════════════════════════════╗");
    println!("║   FAHRENHEIT TO CELSIUS CONVERSION    ║");
    println!("╚════════════════════════════════════════╝");
    println!();
    
    // Print header
    println!("┌──────────────┬──────────────┐");
    println!("│  Fahrenheit  │    Celsius   │");
    println!("├──────────────┼──────────────┤");
    
    // Initialize fahrenheit with starting value
    let mut fahrenheit = START_FAHRENHEIT;
    
    // While loop construction as required
    while fahrenheit <= END_FAHRENHEIT {
        // Calculate Celsius using the formula
        // Note: We use 5.0/9.0 to ensure floating point division
        let celsius = (5.0 / 9.0) * (fahrenheit - 32.0);
        
        // Print the row with proper formatting
        // {:10.1} means: 10 characters wide, 1 decimal place
        println!("│     {:5.0}    │    {:7.1}   │", fahrenheit, celsius);
        
        // Increment fahrenheit by step size
        fahrenheit += STEP_SIZE;
    }
    
    println!("└──────────────┴──────────────┘");
    
    // Display the formula and configuration
    println!("\n╔════════════════════════════════════════╗");
    println!("║              FORMULA USED              ║");
    println!("╚════════════════════════════════════════╝");
    println!("  Celsius = (5/9) × (Fahrenheit - 32)");
    println!();
    println!("  Configuration:");
    println!("  • Start: {}°F", START_FAHRENHEIT);
    println!("  • End:   {}°F", END_FAHRENHEIT);
    println!("  • Step:  {}°F", STEP_SIZE);
    
    // Show key temperature conversions
    println!("\n╔════════════════════════════════════════╗");
    println!("║        KEY TEMPERATURE POINTS          ║");
    println!("╚════════════════════════════════════════╝");
    println!("  •   0°F = -17.8°C (Very cold)");
    println!("  •  32°F =   0.0°C (Water freezes)");
    println!("  •  98.6°F = 37.0°C (Body temperature)");
    println!("  • 212°F = 100.0°C (Water boils)");
    
    // Note about maintenance
    println!("\n╔════════════════════════════════════════╗");
    println!("║         MAINTENANCE FRIENDLY           ║");
    println!("╚════════════════════════════════════════╝");
    println!("  To adjust the range or step size,");
    println!("  simply modify the constants at the");
    println!("  top of the program:");
    println!("  • START_FAHRENHEIT");
    println!("  • END_FAHRENHEIT");
    println!("  • STEP_SIZE");
}
