// Point Module - With Constructor/Destructor Tracking
// ====================================================
// This version includes print statements in constructors
// and destructors to track object lifecycle.
// Also implements Clone trait (Rust's equivalent to copy constructor)

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

// Global counters for tracking constructor/destructor calls
static CONSTRUCTOR_COUNT: AtomicUsize = AtomicUsize::new(0);
static DESTRUCTOR_COUNT: AtomicUsize = AtomicUsize::new(0);
static CLONE_COUNT: AtomicUsize = AtomicUsize::new(0);

// Point struct definition
#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
    id: usize,  // Unique ID to track individual points
}

#[allow(dead_code)]
impl Point {
    // Constructor with coordinates (parameterized constructor)
    pub fn new(x: f64, y: f64) -> Self {
        let count = CONSTRUCTOR_COUNT.fetch_add(1, Ordering::SeqCst);
        let id = count;
        println!("  [CONSTRUCTOR] Creating Point #{} at ({}, {})", id, x, y);
        Point { x, y, id }
    }
    
    // Getters
    pub fn get_x(&self) -> f64 {
        self.x
    }
    
    pub fn get_y(&self) -> f64 {
        self.y
    }
    
    // Setters
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    
    // ToString equivalent
    pub fn to_string(&self) -> String {
        format!("Point #{}({}, {})", self.id, self.x, self.y)
    }
    
    // Distance to origin
    pub fn distance_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    // Distance function - BY VALUE (like C++ Distance(Point p))
    // This will require cloning in Rust to avoid move
    pub fn distance_by_value(self, other: Point) -> f64 {
        println!("  [DISTANCE_BY_VALUE] Taking ownership of Point #{} and Point #{}", 
                 self.id, other.id);
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let result = (dx * dx + dy * dy).sqrt();
        // self and other will be dropped at end of function
        result
    }
    
    // Distance function - BY REFERENCE (like C++ Distance(const Point& p))
    // This is the idiomatic Rust way
    pub fn distance_by_reference(&self, other: &Point) -> f64 {
        println!("  [DISTANCE_BY_REF] Borrowing Point #{} and Point #{}", 
                 self.id, other.id);
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    // Standard distance function (by reference - Rust idiomatic)
    pub fn distance(&self, other: &Point) -> f64 {
        self.distance_by_reference(other)
    }
    
    // Get current counts
    pub fn get_counts() -> (usize, usize, usize) {
        (
            CONSTRUCTOR_COUNT.load(Ordering::SeqCst),
            DESTRUCTOR_COUNT.load(Ordering::SeqCst),
            CLONE_COUNT.load(Ordering::SeqCst)
        )
    }
    
    // Reset counters (for testing)
    pub fn reset_counts() {
        CONSTRUCTOR_COUNT.store(0, Ordering::SeqCst);
        DESTRUCTOR_COUNT.store(0, Ordering::SeqCst);
        CLONE_COUNT.store(0, Ordering::SeqCst);
    }
}

// Default constructor implementation
impl Default for Point {
    fn default() -> Self {
        let count = CONSTRUCTOR_COUNT.fetch_add(1, Ordering::SeqCst);
        let id = count;
        println!("  [DEFAULT CONSTRUCTOR] Creating Point #{} at (0, 0)", id);
        Point { 
            x: 0.0, 
            y: 0.0,
            id 
        }
    }
}

// Clone trait implementation (equivalent to C++ copy constructor)
impl Clone for Point {
    fn clone(&self) -> Self {
        let count = CONSTRUCTOR_COUNT.fetch_add(1, Ordering::SeqCst);
        let clone_count = CLONE_COUNT.fetch_add(1, Ordering::SeqCst);
        let new_id = count;
        println!("  [CLONE/COPY CONSTRUCTOR] Cloning Point #{} to create Point #{} (clone #{})", 
                 self.id, new_id, clone_count);
        Point {
            x: self.x,
            y: self.y,
            id: new_id,
        }
    }
}

// Drop trait implementation (destructor)
impl Drop for Point {
    fn drop(&mut self) {
        let count = DESTRUCTOR_COUNT.fetch_add(1, Ordering::SeqCst);
        println!("  [DESTRUCTOR] Dropping Point #{} at ({}, {}) [destructor call #{}]", 
                 self.id, self.x, self.y, count);
    }
}

// Display trait for nice printing
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point #{}({:.2}, {:.2})", self.id, self.x, self.y)
    }
}

// Additional helper functions for demonstration
#[allow(dead_code)]
impl Point {
    /// Print current constructor/destructor statistics
    pub fn print_statistics() {
        let (constructors, destructors, clones) = Self::get_counts();
        println!("\n╔════════════════════════════════════════╗");
        println!("║         OBJECT STATISTICS              ║");
        println!("╚════════════════════════════════════════╝");
        println!("Total constructor calls: {}", constructors);
        println!("  - Regular/Default: {}", constructors - clones);
        println!("  - Clone (copy):    {}", clones);
        println!("Total destructor calls:  {}", destructors);
        println!("Objects still alive:     {}", constructors - destructors);
        
        if constructors == destructors {
            println!("✓ Constructor and destructor calls match!");
        } else {
            println!("⚠ Mismatch: {} constructors vs {} destructors", 
                     constructors, destructors);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constructor_destructor_balance() {
        Point::reset_counts();
        
        {
            let p1 = Point::new(1.0, 2.0);
            let _p2 = Point::new(3.0, 4.0);
            let _p3 = p1.clone();
            // All points dropped here
        }
        
        let (constructors, destructors, _) = Point::get_counts();
        assert_eq!(constructors, destructors, 
                   "Constructor and destructor calls should match");
    }
    
    #[test]
    fn test_clone_creates_new_object() {
        Point::reset_counts();
        
        let p1 = Point::new(5.0, 6.0);
        let p2 = p1.clone();
        
        assert_eq!(p1.get_x(), p2.get_x());
        assert_eq!(p1.get_y(), p2.get_y());
        assert_ne!(p1.id, p2.id, "Cloned point should have different ID");
    }
}
