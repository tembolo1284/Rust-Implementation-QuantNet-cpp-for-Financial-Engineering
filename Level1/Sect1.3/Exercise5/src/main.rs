// Exercise 5 Requirements:
// ------------------------
// Create a C-program that clearly shows the difference between --i and i--.
//
// Note: In Rust, we have the same pre-decrement and post-decrement concepts,
// but Rust doesn't have -- and ++ operators. We must be more explicit.

fn main() {
    println!("Demonstrating Pre-decrement vs Post-decrement");
    println!("==============================================\n");
    
    // In C: --i (pre-decrement)
    println!("Pre-decrement (--i in C):");
    println!("-------------------------");
    let mut i = 5;
    println!("Initial value: i = {}", i);
    
    // Pre-decrement: decrement first, then use the value
    i -= 1;  // Decrement i first
    let pre_result = i;  // Then use the decremented value
    println!("After pre-decrement: value used = {}, i = {}", pre_result, i);
    println!("(Decrements BEFORE using the value)\n");
    
    // In C: i-- (post-decrement)
    println!("Post-decrement (i-- in C):");
    println!("--------------------------");
    let mut i = 5;
    println!("Initial value: i = {}", i);
    
    // Post-decrement: use the value first, then decrement
    let post_result = i;  // Use the current value first
    i -= 1;  // Then decrement i
    println!("After post-decrement: value used = {}, i = {}", post_result, i);
    println!("(Decrements AFTER using the value)\n");
    
    // Practical example showing the difference
    println!("Practical Example - Array Indexing:");
    println!("------------------------------------");
    let arr = [10, 20, 30, 40, 50];
    
    // Simulating --i behavior
    println!("\nUsing pre-decrement pattern:");
    let mut index = 3;
    println!("Starting at index = {}", index);
    index -= 1;  // Decrement first
    println!("Accessing arr[{}] = {}", index, arr[index]);  // Then use
    println!("Final index = {}", index);
    
    // Simulating i-- behavior
    println!("\nUsing post-decrement pattern:");
    let mut index = 3;
    println!("Starting at index = {}", index);
    let value = arr[index];  // Use first
    index -= 1;  // Then decrement
    println!("Accessed arr[3] = {}, index is now {}", value, index);
    println!("Final index = {}", index);
    
    // Side-by-side comparison
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║              SUMMARY COMPARISON                    ║");
    println!("╠════════════════════════════════════════════════════╣");
    println!("║ --i (pre-decrement):                               ║");
    println!("║   1. Decrement the variable                        ║");
    println!("║   2. Use the NEW (decremented) value               ║");
    println!("║                                                    ║");
    println!("║ i-- (post-decrement):                              ║");
    println!("║   1. Use the CURRENT value                         ║");
    println!("║   2. Decrement the variable                        ║");
    println!("╚════════════════════════════════════════════════════╝");
    
    // C-equivalent code comment
    println!("\n/* Equivalent C code:");
    println!("   int i = 5;");
    println!("   int pre = --i;   // pre gets 4, i becomes 4");
    println!("   ");
    println!("   int j = 5;");
    println!("   int post = j--;  // post gets 5, j becomes 4");
    println!("*/");
}
