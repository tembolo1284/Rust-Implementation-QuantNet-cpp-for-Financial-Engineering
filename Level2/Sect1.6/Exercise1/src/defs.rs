// Exercise 1 - Section 1.6 (Macro Definitions)
// ---------------------------------------------
// This file contains the macro definitions (equivalent to Defs.h)
// In C, we would use #ifndef guards to prevent multiple inclusion
// In Rust, the module system handles this automatically

// Define PRINT1 macro that prints one variable
#[macro_export]
macro_rules! print1 {
    ($x:expr) => {
        println!("Value = {}", $x);
    };
}

// Define PRINT2 macro that prints two variables
#[macro_export]
macro_rules! print2 {
    ($x:expr, $y:expr) => {
        println!("Value1 = {}, Value2 = {}", $x, $y);
    };
}
