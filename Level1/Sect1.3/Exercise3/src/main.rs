// Exercise 3 Requirements:
// ------------------------
// In the following program various operators are used to assign a value to the variable x.
// In this example the string that is passed to printf() has the format specification %d.
// This means that a decimal will be printed in place of %d. This decimal is passed to
// printf() as the second argument. The first argument of printf() must be a string.
// In this example the second argument is the variable x.
// Predict what will be printed on screen (provide a code file with comments stating
// the output for each line).
//
// Note: In Rust, we use println!() with {} as placeholder instead of printf() with %d

fn main() {
    let mut x: i32;
    
    // Expression: -3 + 4 * 5 - 6
    // Order of operations: * first, then left to right for +/-
    // = -3 + 20 - 6
    // = 17 - 6
    // = 11
    x = -3 + 4 * 5 - 6;
    println!("x={}", x);  // Output: x=11
    
    // Expression: 3 + 4 % 5 - 6
    // Order of operations: % first, then left to right for +/-
    // = 3 + 4 - 6  (4 % 5 = 4, since 4 < 5)
    // = 7 - 6
    // = 1
    x = 3 + 4 % 5 - 6;
    println!("x={}", x);  // Output: x=1
    
    // Expression: -3 * 4 % -6 / 5
    // Order of operations: *, %, / have same precedence, left to right
    // = -12 % -6 / 5
    // = 0 / 5  (-12 % -6 = 0, since -12 is divisible by -6)
    // = 0
    // Note: In Rust, % with negative numbers follows truncated division
    // (same as C99 and later)
    x = -3 * 4 % -6 / 5;
    println!("x={}", x);  // Output: x=0
    
    // Expression: (7 + 6) % 5 / 2
    // Order of operations: parentheses first, then % and / left to right
    // = 13 % 5 / 2
    // = 3 / 2  (13 % 5 = 3, remainder when 13 divided by 5)
    // = 1  (integer division: 3 / 2 = 1 with remainder discarded)
    x = (7 + 6) % 5 / 2;
    println!("x={}", x);  // Output: x=1
    
    println!("\nComplete output summary:");
    println!("x=11");
    println!("x=1");
    println!("x=0");
    println!("x=1");
}
