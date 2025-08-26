// Exercise 2 - Section 1.6 (Extended Macro Definitions)
// ------------------------------------------------------
// This file contains all macro definitions including PRINT1, PRINT2,
// MAX2, and MAX3 (equivalent to the complete Defs.h)

// PRINT1 macro from Exercise 1
#[macro_export]
macro_rules! print1 {
    ($x:expr) => {
        println!("Value = {}", $x);
    };
}

// PRINT2 macro from Exercise 1
#[macro_export]
macro_rules! print2 {
    ($x:expr, $y:expr) => {
        println!("Value1 = {}, Value2 = {}", $x, $y);
    };
}

// MAX2 macro that returns the maximum of two values
#[macro_export]
macro_rules! max2 {
    ($x:expr, $y:expr) => {
        {
            let temp_x = $x;
            let temp_y = $y;
            if temp_x > temp_y { temp_x } else { temp_y }
        }
    };
}

// MAX3 macro that returns the maximum of three values
// This macro uses MAX2 internally as required
#[macro_export]
macro_rules! max3 {
    ($x:expr, $y:expr, $z:expr) => {
        max2!(max2!($x, $y), $z)
    };
}
