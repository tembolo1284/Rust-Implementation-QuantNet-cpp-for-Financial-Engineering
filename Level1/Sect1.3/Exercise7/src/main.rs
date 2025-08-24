// Exercise 7 Requirements:
// ------------------------
// Write a C-program that efficiently multiplies a number by a factor 2 to the power n.
// The number to multiply and n are variables, which get a value at the start of the program.
// Clue:
// 1 shift to the left is the same as multiplying by 2.
// 2 shifts to the left are the same as multiplying by 4.
// 3 shifts to the left are the same as multiplying by 8.
//
// Note: Left shift (<<) is an efficient way to multiply by powers of 2

use std::io;

fn main() {
    println!("Efficient Multiplication by 2^n using Bit Shifting");
    println!("===================================================\n");
    
    // Get the number to multiply
    println!("Enter the number to multiply:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let number: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid integer");
    
    // Get the power n
    println!("Enter the power n (to multiply by 2^n):");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let n: u32 = input
        .trim()
        .parse()
        .expect("Please enter a valid non-negative integer");
    
    // Efficient multiplication using left shift
    let result = number << n;
    
    // Also calculate using regular multiplication for comparison
    let power_of_2 = 2_i32.pow(n);
    let regular_result = number * power_of_2;
    
    // Display results
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║                    RESULTS                        ║");
    println!("╚════════════════════════════════════════════════════╝");
    
    println!("\nInput number: {}", number);
    println!("Power n: {}", n);
    println!("Multiplying by: 2^{} = {}", n, power_of_2);
    
    println!("\n--- EFFICIENT METHOD (Bit Shift) ---");
    println!("{} << {} = {}", number, n, result);
    println!("Binary representation:");
    println!("  Original: {:032b}", number);
    println!("  Result:   {:032b}", result);
    
    println!("\n--- REGULAR METHOD (Multiplication) ---");
    println!("{} × {} = {}", number, power_of_2, regular_result);
    
    println!("\n--- VERIFICATION ---");
    println!("Both methods give same result: {}", 
             if result == regular_result { "✓ YES" } else { "✗ NO" });
    
    // Show step-by-step shifting for educational purposes
    if n <= 5 && n > 0 {
        println!("\n╔════════════════════════════════════════════════════╗");
        println!("║              STEP-BY-STEP SHIFTING                ║");
        println!("╚════════════════════════════════════════════════════╝");
        
        let mut temp = number;
        println!("\nStarting with: {}", temp);
        
        for i in 1..=n {
            temp = number << i;
            println!("After {} shift(s) left: {} << {} = {} (× {})", 
                     i, number, i, temp, 2_i32.pow(i));
        }
    }
    
    // Examples table
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║            COMMON EXAMPLES TABLE                  ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("┌─────────┬──────────────┬─────────────────────────┐");
    println!("│ n       │ 2^n          │ Operation               │");
    println!("├─────────┼──────────────┼─────────────────────────┤");
    println!("│ 0       │ 1            │ {} << 0 = {:6}        │", number, number << 0);
    println!("│ 1       │ 2            │ {} << 1 = {:6}        │", number, number << 1);
    println!("│ 2       │ 4            │ {} << 2 = {:6}        │", number, number << 2);
    println!("│ 3       │ 8            │ {} << 3 = {:6}        │", number, number << 3);
    println!("│ 4       │ 16           │ {} << 4 = {:6}        │", number, number << 4);
    println!("│ 5       │ 32           │ {} << 5 = {:6}        │", number, number << 5);
    println!("└─────────┴──────────────┴─────────────────────────┘");
    
    // Performance note
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║               WHY USE BIT SHIFTING?                ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("• Bit shifting is a single CPU instruction");
    println!("• Much faster than multiplication on most processors");
    println!("• Compiler often optimizes × 2^n to << n automatically");
    println!("• Useful for low-level programming and embedded systems");
    
    // Warning about overflow
    if n >= 16 {
        println!("\n⚠️  WARNING: Large shifts may cause overflow!");
        println!("   For i32, shifting left by {} or more bits", 32);
        println!("   can lead to unexpected results or overflow.");
    }
    
}
