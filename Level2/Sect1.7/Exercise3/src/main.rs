// Exercise 3 - Section 1.7 Requirements:
// ---------------------------------------
// Predict what will be printed on the screen
// This exercise demonstrates various ways to access array elements
// using pointers and array indexing in C
//
// Note: Rust doesn't support all the same pointer arithmetic as C,
// but we'll demonstrate equivalent concepts

// Macro to print decimal (similar to PRD macro)
macro_rules! prd {
    ($a:expr) => {
        print!("{}", $a);
    };
}

// Macro to print newline (similar to NL macro)
macro_rules! nl {
    () => {
        println!();
    };
}

fn main() {
    // Create and initialize array
    let a = [0, 1, 2, 3, 4];
    
    println!("Array Access Patterns Demonstration");
    println!("====================================\n");
    println!("Array a = {:?}\n", a);
    
    // Loop 1: Standard array indexing
    println!("╔════════════════════════════════════════╗");
    println!("║   Loop 1: Standard Array Indexing     ║");
    println!("╚════════════════════════════════════════╝");
    println!("Code: for (i=0; i<=4; i++) PRD(a[i])");
    print!("Output: ");
    for i in 0..=4 {
        prd!(a[i]);
    }
    nl!();
    println!("Explanation: Direct array indexing a[0], a[1], ..., a[4]");
    println!("Result: 01234\n");
    
    // Loop 2: Pointer iteration
    println!("╔════════════════════════════════════════╗");
    println!("║   Loop 2: Pointer Iteration           ║");
    println!("╚════════════════════════════════════════╝");
    println!("Code: for (p=&a[0]; p<=&a[4]; p++) PRD(*p)");
    print!("Output: ");
    for element in &a {
        prd!(element);
    }
    nl!();
    println!("Explanation: Pointer moves through array, dereferencing each");
    println!("Result: 01234\n");
    
    // Loop 3: Pointer as array base
    println!("╔════════════════════════════════════════╗");
    println!("║   Loop 3: Pointer as Array Base       ║");
    println!("╚════════════════════════════════════════╝");
    println!("Code: for (p=&a[0], i=0; i<=4; i++) PRD(p[i])");
    print!("Output: ");
    let p = &a[0] as *const i32;
    for i in 0..=4 {
        unsafe {
            prd!(*p.offset(i as isize));
        }
    }
    nl!();
    println!("Explanation: p points to start, p[i] accesses i-th element");
    println!("Result: 01234\n");
    
    // Loop 4: Double increment (TRICKY!)
    println!("╔════════════════════════════════════════╗");
    println!("║   Loop 4: Double Increment (TRICKY!)  ║");
    println!("╚════════════════════════════════════════╝");
    println!("Code: for (p=a, i=0; p+i<=a+4; p++, i++) PRD(*(p+i))");
    print!("Output: ");
    let mut p_offset = 0;
    let mut i = 0;
    while p_offset + i <= 4 {
        prd!(a[p_offset + i]);
        p_offset += 1;
        i += 1;
    }
    nl!();
    println!("Explanation: BOTH p and i increment each iteration!");
    println!("  Iteration 1: p=a+0, i=0 → *(a+0+0) = a[0] = 0");
    println!("  Iteration 2: p=a+1, i=1 → *(a+1+1) = a[2] = 2");
    println!("  Iteration 3: p=a+2, i=2 → *(a+2+2) = a[4] = 4");
    println!("Result: 024\n");
    
    // Loop 5: Reverse iteration
    println!("╔════════════════════════════════════════╗");
    println!("║   Loop 5: Reverse Iteration           ║");
    println!("╚════════════════════════════════════════╝");
    println!("Code: for (p=a+4; p>=a; p--) PRD(*p)");
    print!("Output: ");
    for i in (0..=4).rev() {
        prd!(a[i]);
    }
    nl!();
    println!("Explanation: Start at end, move backwards");
    println!("Result: 43210\n");
    
    // Loop 6: Negative indexing
    println!("╔════════════════════════════════════════╗");
    println!("║   Loop 6: Negative Indexing           ║");
    println!("╚════════════════════════════════════════╝");
    println!("Code: for (p=a+4, i=0; i<=4; i++) PRD(p[-i])");
    print!("Output: ");
    for i in 0..=4 {
        prd!(a[4 - i]);
    }
    nl!();
    println!("Explanation: p points to end, p[-i] goes backwards");
    println!("  p[-0] = a[4] = 4, p[-1] = a[3] = 3, etc.");
    println!("Result: 43210\n");
    
    // Loop 7: Pointer arithmetic for indexing
    println!("╔════════════════════════════════════════╗");
    println!("║   Loop 7: Pointer Difference Index    ║");
    println!("╚════════════════════════════════════════╝");
    println!("Code: for (p=a+4; p>=a; p--) PRD(a[p-a])");
    print!("Output: ");
    for i in (0..=4).rev() {
        let index = i;  // p-a gives us the index
        prd!(a[index]);
    }
    nl!();
    println!("Explanation: (p-a) calculates index from pointer difference");
    println!("  When p=a+4: a[4-0]=a[4]=4, When p=a+3: a[3-0]=a[3]=3");
    println!("Result: 43210\n");
    
    // Summary
    println!("╔════════════════════════════════════════╗");
    println!("║           COMPLETE OUTPUT              ║");
    println!("╚════════════════════════════════════════╝");
    println!("01234   (Loop 1: Standard indexing)");
    println!("01234   (Loop 2: Pointer iteration)");
    println!();
    println!("01234   (Loop 3: Pointer as base)");
    println!("024     (Loop 4: Double increment - TRICKY!)");
    println!();
    println!("43210   (Loop 5: Reverse iteration)");
    println!("43210   (Loop 6: Negative indexing)");
    println!("43210   (Loop 7: Pointer difference)");
    
    // Pointer arithmetic explanation
    println!("\n╔════════════════════════════════════════╗");
    println!("║      POINTER ARITHMETIC CONCEPTS       ║");
    println!("╚════════════════════════════════════════╝");
    println!("In C:");
    println!("• p++ moves pointer to next element");
    println!("• p[i] is equivalent to *(p+i)");
    println!("• p[-i] is equivalent to *(p-i)");
    println!("• (p-a) gives the number of elements between pointers");
    println!();
    println!("Key insight for Loop 4:");
    println!("• Both p and i increment together");
    println!("• This causes us to skip elements!");
    println!("• p+i advances by 2 positions each iteration");
    
    // Rust vs C differences
    println!("\n╔════════════════════════════════════════╗");
    println!("║         RUST VS C DIFFERENCES          ║");
    println!("╚════════════════════════════════════════╝");
    println!("C allows:");
    println!("• Direct pointer arithmetic (p++, p+i)");
    println!("• Negative array indexing (p[-i])");
    println!("• Pointer comparison (p<=&a[4])");
    println!();
    println!("Rust requires:");
    println!("• unsafe blocks for raw pointer operations");
    println!("• Explicit indexing or iterators");
    println!("• Bounds checking by default");
}

