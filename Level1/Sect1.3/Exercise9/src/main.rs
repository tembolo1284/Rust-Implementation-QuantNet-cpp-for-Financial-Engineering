// Exercise 9 Requirements:
// ------------------------
// Predict what the following program prints on screen (provide a code file
// with comments stating the output for each line).
// The program uses conditional expressions and compound assignment operators.
//
// Note: Rust doesn't have post-increment (++), so we simulate it explicitly

fn main() {
    println!("Conditional Expressions and Compound Assignments");
    println!("================================================\n");
    
    let mut x = 1;
    let mut y = 1;
    let _z = 1;  // z is declared but not used in the C program
    
    println!("Initial values: x={}, y={}, z=1\n", x, y);
    
    // Expression: x += y += x
    // This is right-to-left associative: x += (y += x)
    // Step 1: y += x → y = y + x → y = 1 + 1 = 2
    // Step 2: x += y → x = x + y → x = 1 + 2 = 3
    // After this: x=3, y=2
    y += x;  // y = 1 + 1 = 2
    x += y;  // x = 1 + 2 = 3
    println!("After x += y += x:");
    println!("x = {}, y = {}\n", x, y);
    
    // Line 1: printf("%d\n\n", (x<y)?y:x);
    // Condition: x < y → 3 < 2 → false
    // Result: Since false, return x → 3
    let result1 = if x < y { y } else { x };
    println!("{}", result1);  // Output: 3
    println!();  // Extra newline as in original
    println!("// Number 1: {} (x={} is not less than y={}, so returns x)", result1, x, y);
    
    // Line 2: printf("%d\n", (x<y)?x++:y++);
    // Condition: x < y → 3 < 2 → false
    // Result: Since false, execute y++
    // y++ returns current value of y (2), then increments y to 3
    // So prints 2, and after this y=3
    let result2 = if x < y { 
        let temp = x;
        x += 1;
        temp
    } else { 
        let temp = y;  // Store current value (2)
        y += 1;         // Increment y to 3
        temp            // Return original value (2)
    };
    println!("{}", result2);  // Output: 2
    println!("// Number 2: {} (condition false, so y++ executes: returns 2, y becomes 3)", result2);
    
    // Line 3: printf("%d\n", x);
    // x is still 3 (wasn't modified by the previous conditional)
    println!("{}", x);  // Output: 3
    println!("// Number 3: {} (x unchanged)", x);
    
    // Line 4: printf("%d\n", y);
    // y is now 3 (was incremented from 2 to 3)
    println!("{}", y);  // Output: 3
    println!("// Number 4: {} (y was incremented from 2 to 3)", y);
    
    // Summary
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║               PREDICTED OUTPUT                     ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("3");
    println!("");
    println!("2");
    println!("3");
    println!("3");
    
    // Detailed step-by-step breakdown
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║            STEP-BY-STEP ANALYSIS                   ║");
    println!("╚════════════════════════════════════════════════════╝");
    
    println!("\nInitial State:");
    println!("  x=1, y=1, z=1");
    
    println!("\nStep 1: x += y += x");
    println!("  └─ Right-to-left evaluation");
    println!("  └─ First: y += x → y = 1 + 1 = 2");
    println!("  └─ Then: x += y → x = 1 + 2 = 3");
    println!("  └─ Result: x=3, y=2");
    
    println!("\nStep 2: (x<y)?y:x");
    println!("  └─ Condition: 3 < 2 → FALSE");
    println!("  └─ Returns: x = 3");
    println!("  └─ Prints: 3");
    
    println!("\nStep 3: (x<y)?x++:y++");
    println!("  └─ Condition: 3 < 2 → FALSE");
    println!("  └─ Executes: y++ (post-increment)");
    println!("  └─ Returns: 2 (current value of y)");
    println!("  └─ Side effect: y becomes 3");
    println!("  └─ Prints: 2");
    
    println!("\nStep 4: Print x");
    println!("  └─ x is still 3");
    println!("  └─ Prints: 3");
    
    println!("\nStep 5: Print y");
    println!("  └─ y is now 3 (after increment)");
    println!("  └─ Prints: 3");
    
    // Key concepts
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║                 KEY CONCEPTS                       ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("• Compound assignments are right-associative");
    println!("• Post-increment (i++) returns current value, then increments");
    println!("• Ternary operator (?:) only evaluates the chosen branch");
    println!("• Side effects (like ++) happen even in conditional expressions");
    
}
