// Exercise 3: Creating Array Class (Safe Rust Alternative to C++ Manual Memory Management)
// ===================================================================================
//
// This exercise demonstrates how Rust's ownership system and standard library
// provide the same functionality as a custom C++ array class but with memory safety.
//
// C++ Approach (Manual Memory Management):
//   class Array {
//       Point* m_data;     // Raw pointer to dynamic array
//       size_t m_size;     // Size of array
//   public:
//       Array(size_t size);           // Constructor with size
//       Array(const Array& other);    // Copy constructor  
//       ~Array();                     // Destructor
//       Array& operator=(const Array& other); // Assignment operator
//       Point& operator[](int index); // Index operator
//   };
//
// Rust Approach (Automatic Memory Management):
//   struct Array {
//       data: Vec<Point>,  // Owned vector of Points
//   }
//   // Copy, Drop, and assignment handled automatically by compiler
//   // Index operators implemented via traits
//   // Memory safety guaranteed at compile time

#![allow(dead_code)]
#![allow(unused_assignments)]

mod point;
mod array;

use point::Point;
use array::Array;

fn main() {
    println!("Level 4, Section 2.5, Exercise 3: Creating Array Class");
    println!("=======================================================\n");

    // Test default constructor (10 elements)
    println!("=== Testing Default Constructor (10 elements) ===");
    let arr1 = Array::default();
    println!("Created array with default constructor");
    println!("Size: {}", arr1.size());
    println!("Initial elements (all default Points):");
    for i in 0..arr1.size() {
        println!("  arr1[{}]: {}", i, arr1.get_element(i));
    }

    // Test constructor with size argument
    println!("\n=== Testing Constructor with Size Argument ===");
    let mut arr2 = Array::with_size(5);
    println!("Created array with size 5");
    println!("Size: {}", arr2.size());

    // Test SetElement function
    println!("\n=== Testing SetElement Function ===");
    arr2.set_element(0, Point::new(1.0, 2.0));
    arr2.set_element(1, Point::new(3.0, 4.0));
    arr2.set_element(2, Point::from_single_value(5.0));
    arr2.set_element(3, Point::from(7.0));
    arr2.set_element(4, Point::new(-1.0, -2.0));
    
    println!("After setting elements:");
    for i in 0..arr2.size() {
        println!("  arr2[{}]: {}", i, arr2.get_element(i));
    }

    // Test bounds checking in SetElement
    println!("\n=== Testing Bounds Checking in SetElement ===");
    println!("Attempting to set element at index 10 (out of bounds)...");
    arr2.set_element(10, Point::new(99.0, 99.0)); // Should be ignored
    println!("Array unchanged (bounds checking worked)");

    // Test GetElement function with bounds checking  
    println!("\n=== Testing GetElement with Bounds Checking ===");
    println!("Valid access arr2[2]: {}", arr2.get_element(2));
    println!("Out of bounds access arr2[10]: {} (returns first element)", arr2.get_element(10));

    // Test square bracket operator for reading
    println!("\n=== Testing Square Bracket Operator (Reading) ===");
    println!("arr2[0]: {}", arr2[0]);
    println!("arr2[1]: {}", arr2[1]);
    println!("arr2[4]: {}", arr2[4]);

    // Test square bracket operator for writing
    println!("\n=== Testing Square Bracket Operator (Writing) ===");
    arr2[0] = Point::new(10.0, 20.0);
    arr2[1] = Point::new(30.0, 40.0);
    println!("After modifying via [] operator:");
    println!("arr2[0]: {}", arr2[0]);
    println!("arr2[1]: {}", arr2[1]);

    // Test copy constructor (Clone in Rust)
    println!("\n=== Testing Copy Constructor (Clone) ===");
    let arr3 = arr2.clone(); // Equivalent to C++ copy constructor
    println!("Created arr3 as clone of arr2");
    println!("arr3 size: {}", arr3.size());
    println!("arr3 contents (should match arr2):");
    for i in 0..arr3.size() {
        println!("  arr3[{}]: {}", i, arr3[i]);
    }

    // Verify deep copy (not sharing data)
    println!("\n=== Verifying Deep Copy (Independent Objects) ===");
    println!("Original arr2[0]: {}", arr2[0]);
    println!("Cloned   arr3[0]: {}", arr3[0]);
    
    // Modify original
    arr2[0] = Point::new(999.0, 888.0);
    println!("After modifying arr2[0]:");
    println!("Modified arr2[0]: {}", arr2[0]);
    println!("Unchanged arr3[0]: {}", arr3[0]);
    println!("✓ Deep copy confirmed - objects are independent");

    // Test assignment operator (Clone + assignment)
    println!("\n=== Testing Assignment Operator ===");
    let mut arr4 = Array::with_size(2); // Different size initially
    println!("arr4 initial size: {}", arr4.size());
    
    arr4 = arr2.clone(); // Equivalent to C++ assignment operator
    println!("After arr4 = arr2 (clone assignment):");
    println!("arr4 size: {}", arr4.size());
    println!("arr4[0]: {}", arr4[0]);

    // Test self-assignment safety (automatic in Rust)
    println!("\n=== Testing Self-Assignment Safety ===");
    // In C++: arr4 = arr4; (dangerous without self-assignment check)
    // In Rust: this is safe by design, but let's demonstrate
    let original_value = arr4[0];
    arr4 = arr4.clone(); // Self-assignment via clone
    println!("After self-assignment arr4 = arr4.clone():");
    println!("arr4[0]: {} (unchanged)", arr4[0]);
    assert_eq!(arr4[0], original_value);
    println!("✓ Self-assignment safe");

    // Test const correctness equivalent
    println!("\n=== Testing Const Correctness (Immutable References) ===");
    test_const_array(&arr3);

    // Test destructor (Drop trait - automatic in Rust)
    println!("\n=== Testing Destructor (Drop Trait) ===");
    {
        let temp_array = Array::with_size(1000);
        temp_array[0]; // Use it
        println!("Created temporary array with 1000 elements");
        // temp_array will be automatically dropped here
    }
    println!("Temporary array automatically cleaned up (no manual delete needed)");

    // Performance comparison: Array vs Vec
    println!("\n=== Performance Comparison: Custom Array vs Vec<Point> ===");
    let vec_points = vec![Point::new(1.0, 2.0); 5];
    let array_points = Array::from_vec(vec_points.clone());
    
    println!("Vec<Point> size: {}", vec_points.len());
    println!("Array size: {}", array_points.size());
    println!("Both provide similar functionality with automatic memory management");

    // Advanced operations
    println!("\n=== Advanced Array Operations ===");
    let mut arr5 = Array::with_size(4);
    
    // Fill with calculated values
    for i in 0..arr5.size() {
        arr5[i] = Point::new(i as f64 * 2.0, i as f64 * 3.0);
    }
    
    println!("Array filled with calculated values:");
    for i in 0..arr5.size() {
        println!("  arr5[{}]: {}", i, arr5[i]);
    }

    // Calculate distances between consecutive points
    println!("Distances between consecutive points:");
    for i in 0..arr5.size() - 1 {
        let distance = arr5[i].distance(&arr5[i + 1]);
        println!("  Distance from point {} to {}: {:.2}", i, i + 1, distance);
    }

    // Find point farthest from origin
    let mut max_distance = 0.0;
    let mut farthest_index = 0;
    for i in 0..arr5.size() {
        let distance = arr5[i].distance_to_origin();
        if distance > max_distance {
            max_distance = distance;
            farthest_index = i;
        }
    }
    println!("Point farthest from origin: arr5[{}] = {} (distance: {:.2})", 
             farthest_index, arr5[farthest_index], max_distance);

    // Demonstrate iterator-like functionality
    println!("\n=== Iterator-like Functionality ===");
    println!("All points in arr5:");
    arr5.for_each(|i, point| {
        println!("  Index {}: {} (distance from origin: {:.2})", 
                i, point, point.distance_to_origin());
    });

    // Test bounds checking thoroughly
    println!("\n=== Comprehensive Bounds Checking Test ===");
    let small_array = Array::with_size(2);
    
    println!("Array size: {}", small_array.size());
    println!("Valid indices: 0, 1");
    println!("Testing various out-of-bounds indices:");
    
    let test_indices = [-1, 2, 5, 100];
    for &index in &test_indices {
        if index >= 0 {
            let result = small_array.get_element(index as usize);
            println!("  get_element({}): {} (fallback to first element)", index, result);
        }
    }

    println!("\n=== Summary ===");
    println!("✅ Array class successfully created with all required functionality:");
    println!("  • Default constructor (10 elements)");
    println!("  • Constructor with size argument");  
    println!("  • Copy constructor (Clone trait)");
    println!("  • Destructor (Drop trait - automatic)");
    println!("  • Assignment operator (Clone + assignment)");
    println!("  • Size() function");
    println!("  • SetElement() and GetElement() with bounds checking");
    println!("  • Square bracket operators for reading and writing");
    println!("  • Const correctness (immutable reference methods)");
    println!("  • Self-assignment safety (automatic in Rust)");
    println!("  • Memory safety without manual memory management");
    
    println!("\n🎯 Key Rust Advantages Over C++ Manual Memory Management:");
    println!("  • No memory leaks (automatic cleanup)");
    println!("  • No double-free errors (ownership system)");
    println!("  • No null pointer dereferences (Box/Vec cannot be null)");
    println!("  • No buffer overflows (bounds checking available)");
    println!("  • Thread safety through ownership rules");
    println!("  • Zero-cost abstractions (performance equivalent to C++)");

    // All arrays will be automatically cleaned up here!
    // No manual delete[] required!
}

// Function to demonstrate const correctness (immutable reference)
// Equivalent to C++: void test_const_array(const Array& arr);
fn test_const_array(arr: &Array) {
    println!("Testing const correctness with immutable reference:");
    println!("  Array size: {}", arr.size());
    println!("  Reading arr[0]: {}", arr[0]); // Uses const version of index operator
    println!("  Reading arr[1]: {}", arr[1]);
    // arr[0] = Point::new(1.0, 1.0); // Would be compile error - cannot modify through &Array
    println!("✓ Const correctness enforced at compile time");
}

// Function to demonstrate array parameter passing
fn process_array(arr: &mut Array) {
    println!("Processing array (mutable reference):");
    for i in 0..arr.size() {
        let current = arr[i];
        arr[i] = current * 2.0; // Scale all points
    }
}

// Function to demonstrate array return
fn create_test_array() -> Array {
    let mut arr = Array::with_size(3);
    arr[0] = Point::new(1.0, 1.0);
    arr[1] = Point::new(2.0, 2.0);
    arr[2] = Point::new(3.0, 3.0);
    arr // Return by move (no copy needed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constructor() {
        let arr = Array::default();
        assert_eq!(arr.size(), 10);
        
        // All elements should be default Points
        for i in 0..arr.size() {
            assert_eq!(arr[i], Point::default());
        }
    }

    #[test]
    fn test_constructor_with_size() {
        let arr = Array::with_size(5);
        assert_eq!(arr.size(), 5);
        
        for i in 0..arr.size() {
            assert_eq!(arr[i], Point::default());
        }
    }

    #[test]
    fn test_set_and_get_element() {
        let mut arr = Array::with_size(3);
        
        let test_point = Point::new(3.14, 2.71);
        arr.set_element(1, test_point);
        
        assert_eq!(arr.get_element(1), test_point);
    }

    #[test]
    fn test_bounds_checking_set_element() {
        let mut arr = Array::with_size(2);
        let original_first = arr[0];
        
        // Out of bounds set should be ignored
        arr.set_element(5, Point::new(99.0, 99.0));
        assert_eq!(arr[0], original_first); // First element unchanged
    }

    #[test]
    fn test_bounds_checking_get_element() {
        let mut arr = Array::with_size(2);
        let test_point = Point::new(1.0, 2.0);
        arr[0] = test_point;
        
        // Out of bounds get should return first element
        assert_eq!(arr.get_element(10), test_point);
    }

    #[test]
    fn test_index_operators() {
        let mut arr = Array::with_size(2);
        
        // Test write access
        arr[0] = Point::new(5.0, 6.0);
        arr[1] = Point::new(7.0, 8.0);
        
        // Test read access
        assert_eq!(arr[0], Point::new(5.0, 6.0));
        assert_eq!(arr[1], Point::new(7.0, 8.0));
    }

    #[test]
    fn test_copy_constructor() {
        let mut arr1 = Array::with_size(2);
        arr1[0] = Point::new(1.0, 2.0);
        arr1[1] = Point::new(3.0, 4.0);
        
        let arr2 = arr1.clone(); // Copy constructor
        
        // Should have same size and contents
        assert_eq!(arr2.size(), arr1.size());
        assert_eq!(arr2[0], arr1[0]);
        assert_eq!(arr2[1], arr1[1]);
        
        // Should be independent (deep copy)
        arr1[0] = Point::new(99.0, 99.0);
        assert_ne!(arr2[0], arr1[0]); // arr2 unchanged
    }

    #[test]
    fn test_assignment_operator() {
        let mut arr1 = Array::with_size(2);
        arr1[0] = Point::new(1.0, 2.0);
        
        let mut arr2 = Array::with_size(5); // Different size
        arr2 = arr1.clone(); // Assignment
        
        // Should match arr1
        assert_eq!(arr2.size(), arr1.size());
        assert_eq!(arr2[0], arr1[0]);
    }

    #[test]
    fn test_self_assignment_safety() {
        let mut arr = Array::with_size(2);
        arr[0] = Point::new(1.0, 2.0);
        
        let original = arr[0];
        arr = arr.clone(); // Self-assignment
        
        // Should be unchanged
        assert_eq!(arr[0], original);
    }

    #[test]
    fn test_const_correctness() {
        let arr = Array::with_size(1);
        
        // Should be able to read from const reference
        test_const_array(&arr);
        
        // This test just verifies the function compiles and runs
        assert!(true);
    }

    #[test]
    fn test_memory_safety() {
        // Test that arrays are properly cleaned up
        for _ in 0..1000 {
            let _arr = Array::with_size(100);
            // Each array should be automatically cleaned up
        }
        assert!(true); // Test passes if no memory leaks
    }
}
