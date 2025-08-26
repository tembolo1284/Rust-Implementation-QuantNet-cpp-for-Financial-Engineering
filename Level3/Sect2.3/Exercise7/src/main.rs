// Exercise 7: Inline Functions
// ============================
// C++: Inline functions copy code at call site instead of function calls
// Rust: Compiler automatically decides, but we can hint with #[inline]
//
// C++ has two styles:
// - Default inline: defined inside class declaration
// - Normal inline: defined outside with 'inline' keyword
//
// Rust: All in impl blocks, use #[inline] attribute for hints

mod point;

use point::Point;

fn main() {
    println!("Exercise 7: Inline Functions");
    println!("============================\n");
    
    println!("=== C++ vs Rust Inlining ===");
    println!("C++:");
    println!("  - Manual inline keyword");
    println!("  - Default inline (inside class)");
    println!("  - Normal inline (outside class)");
    println!();
    println!("Rust:");
    println!("  - Compiler auto-optimizes");
    println!("  - #[inline] hint");
    println!("  - #[inline(always)] force");
    println!("  - #[inline(never)] prevent\n");
    
    // Test inlined getters and setters
    let mut p = Point::new(3.14, 2.71);
    
    println!("=== Testing Inlined Functions ===");
    println!("Initial point: {}", p);
    
    // These getter calls should be inlined
    let x = p.x();  // #[inline] hint
    let y = p.y();  // #[inline] hint
    println!("Got x: {}, y: {}", x, y);
    
    // These setter calls should be inlined
    p.set_x(10.0);  // #[inline(always)]
    p.set_y(20.0);  // #[inline(always)]
    println!("After setting: {}", p);
    
    // Performance test with many calls
    println!("\n=== Performance Test ===");
    let start = std::time::Instant::now();
    
    let mut _sum = 0.0;
    for _ in 0..1_000_000 {
        _sum += p.x() + p.y();  // Inlined getters
        p.set_x(p.x() + 0.001);  // Inlined setter
    }
    
    let duration = start.elapsed();
    println!("1M getter/setter calls took: {:?}", duration);
    println!("(Inlining makes these calls faster)");
    
    println!("\n=== Build Optimization Levels ===");
    println!("Debug build (cargo build):");
    println!("  - Minimal inlining");
    println!("  - #[inline(always)] still inlines");
    println!();
    println!("Release build (cargo build --release):");
    println!("  - Aggressive inlining");
    println!("  - Compiler decides best optimizations");
    println!("  - Much faster execution");
    
    println!("\n=== When to Use #[inline] ===");
    println!("✓ Small, frequently called functions");
    println!("✓ Simple getters/setters");
    println!("✓ Functions in hot loops");
    println!("✗ Large functions (code bloat)");
    println!("✗ Recursive functions");
    println!("✗ Functions with complex logic");
}
