// Exercise 8 Requirements:
// ------------------------
// The following program uses assignment-operators. Predict what will be printed
// on screen (provide a code file with comments stating the output for each line).
// The operators + and == have a higher priority than the assignment-operators.
//
// Note: In Rust, we need to be explicit about variable initialization and mutations

fn main() {
    println!("Assignment Operators and Precedence");
    println!("====================================\n");
    
    let mut x = 2;
    let y: i32;
    let z: i32;
    
    println!("Initial: x = {}", x);
    
    // Expression: x *= 3 + 2
    // Precedence: + has higher priority than *=
    // So this becomes: x *= (3 + 2)
    // Which is: x *= 5
    // Which means: x = x * 5
    // x = 2 * 5 = 10
    x *= 3 + 2;
    println!("After x *= 3 + 2:");
    println!("x={}", x);  // Output: x=10
    println!("Explanation: + has higher precedence, so 3+2=5, then x = 2*5 = 10\n");
    
    // Expression: x *= y = z = 4
    // Assignment operators are right-associative
    // So this evaluates as: x *= (y = (z = 4))
    // First: z = 4 (z gets 4, expression value is 4)
    // Then: y = 4 (y gets 4, expression value is 4)
    // Finally: x *= 4, which means x = x * 4
    // x = 10 * 4 = 40
    z = 4;
    y = z;  // In Rust, we need to assign separately for clarity
    x *= y;
    println!("After x *= y = z = 4:");
    println!("x={}", x);  // Output: x=40
    println!("Explanation: z=4, y=4, then x = 10*4 = 40");
    println!("(Note: y and z are both 4 now)\n");
    
    // Expression: x = y == z
    // Precedence: == has higher priority than =
    // So this becomes: x = (y == z)
    // y == z evaluates to: 4 == 4, which is true
    // In C, true is represented as 1
    // So x = 1
    x = if y == z { 1 } else { 0 };  // Rust doesn't implicitly convert bool to int
    println!("After x = y == z:");
    println!("x={}", x);  // Output: x=1
    println!("Explanation: == has higher precedence, y==z is true (4==4), true→1 in C\n");
    
    // Summary of outputs
    println!("╔════════════════════════════════════════════════════╗");
    println!("║                 PREDICTED OUTPUT                   ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("x=10");
    println!("x=40");
    println!("x=1");
    
    // Detailed breakdown
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║              DETAILED BREAKDOWN                    ║");
    println!("╚════════════════════════════════════════════════════╝");
    
    println!("\n1. x *= 3 + 2");
    println!("   └─ Precedence: + before *=");
    println!("   └─ Evaluation: x *= (3 + 2) → x *= 5");
    println!("   └─ Result: x = 2 * 5 = 10");
    
    println!("\n2. x *= y = z = 4");
    println!("   └─ Right-to-left associativity");
    println!("   └─ Step 1: z = 4");
    println!("   └─ Step 2: y = 4");
    println!("   └─ Step 3: x *= 4");
    println!("   └─ Result: x = 10 * 4 = 40");
    
    println!("\n3. x = y == z");
    println!("   └─ Precedence: == before =");
    println!("   └─ Evaluation: x = (y == z) → x = (4 == 4)");
    println!("   └─ Boolean result: true → 1 in C");
    println!("   └─ Result: x = 1");
    
    // Operator precedence table
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║            OPERATOR PRECEDENCE TABLE               ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("┌──────────────┬─────────────────────────────────────┐");
    println!("│ Priority     │ Operators                           │");
    println!("├──────────────┼─────────────────────────────────────┤");
    println!("│ Higher       │ + - (arithmetic)                    │");
    println!("│      ↑       │ == != < > <= >= (comparison)       │");
    println!("│      ↓       │ = += -= *= /= %= (assignment)      │");
    println!("│ Lower        │ (right-to-left associative)        │");
    println!("└──────────────┴─────────────────────────────────────┘");
    
    // C/Rust differences note
    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║              C vs RUST DIFFERENCES                 ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("• C allows: x *= y = z = 4 (chained assignments)");
    println!("• Rust prefers explicit separate assignments");
    println!("• C implicitly converts bool to int (true→1, false→0)");
    println!("• Rust requires explicit conversion with if/else");
    
}
