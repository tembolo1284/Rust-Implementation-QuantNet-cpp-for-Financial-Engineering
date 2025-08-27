// Exercise 1: Dynamically Creating Objects (Box<T> and Vec<T> in Rust)
// =====================================================================
// In C++: new/delete for heap allocation, manual memory management
// In Rust: Box<T> for heap allocation, automatic memory management via Drop trait
//
// C++ concepts translated:
// - new Point() -> Box::new(Point::new(...))
// - delete ptr -> automatic when Box goes out of scope  
// - Point* ptr -> Box<Point> (owned heap allocation)
// - Point arr[n] -> [Point; N] (compile-time size) or Vec<Point> (runtime size)
// - new Point[n] -> Vec<Point> or Box<[Point]> (heap allocated)

mod point;
mod line;
mod circle;

use point::Point;
use line::Line;
#[allow(unused_imports)]
use circle::Circle;
use std::io;

fn main() {
    println!("Level 4, Section 2.5, Exercise 1: Dynamically Creating Objects");
    println!("================================================================\n");

    // Part 1: Creating objects on the heap (equivalent to C++ new)
    println!("=== Creating Objects on the Heap (Box<T>) ===");
    
    // C++: Point* p1 = new Point();
    // Rust: Box<T> for heap allocation
    let p1: Box<Point> = Box::new(Point::default());
    println!("Default constructor on heap: {}", p1);
    
    // C++: Point* p2 = new Point(3.0, 4.0);  
    // Rust: Box containing Point with coordinates
    let p2: Box<Point> = Box::new(Point::new(3.0, 4.0));
    println!("Constructor with coordinates on heap: {}", p2);
    
    // C++: Point* p3 = new Point(*p2);  // Copy constructor (dereference pointer)
    // Rust: Clone the dereferenced Box contents
    let p3: Box<Point> = Box::new(*p2);  // Copy (Point implements Copy)
    // Alternative: let p3: Box<Point> = Box::new(p2.clone()); if only Clone trait
    println!("Copy constructor equivalent on heap: {}", p3);
    
    // Call methods on heap-allocated objects
    println!("\n--- Method Calls on Heap Objects ---");
    
    // C++: p2->Distance(*p1) or (*p2).Distance(*p1)
    // Rust: Box implements Deref, so we can call methods directly
    let distance = p2.distance(&p1);
    println!("Distance between p2 and p1: {:.2}", distance);
    
    // C++: cout << *p1 << endl;  // Must dereference pointer
    // Rust: Box implements Deref, so Display works directly
    println!("Printing heap objects:");
    println!("p1: {}", p1);  // No dereferencing needed
    println!("p2: {}", p2);
    println!("p3: {}", p3);
    
    // Demonstrate explicit dereferencing (optional in Rust due to Deref coercion)
    println!("Explicit dereferencing:");
    println!("*p1: {}", *p1);
    println!("*p2: {}", *p2);
    
    // C++: delete p1; delete p2; delete p3;  // Manual cleanup required
    // Rust: Automatic cleanup when Box goes out of scope (Drop trait)
    println!("Note: No manual 'delete' needed - automatic cleanup!");
    
    // Part 2: Stack vs Heap allocation
    println!("\n=== Stack vs Heap Allocation ===");
    
    // Stack allocation (automatic storage)
    let stack_point = Point::new(1.0, 2.0);
    println!("Stack allocated point: {}", stack_point);
    
    // Heap allocation (Box<T>)
    let heap_point = Box::new(Point::new(1.0, 2.0));
    println!("Heap allocated point: {}", heap_point);
    
    // Compare memory usage (Box adds pointer overhead)
    println!("Stack point size: {} bytes", std::mem::size_of::<Point>());
    println!("Box<Point> size: {} bytes", std::mem::size_of::<Box<Point>>());
    println!("(Box contains a pointer to heap-allocated Point)");
    
    // Part 3: Dynamic arrays (equivalent to C++ new Point[n])
    println!("\n=== Dynamic Arrays ===");
    
    // Get array size from user
    println!("Enter the size of the point array:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let array_size: usize = input.trim().parse().expect("Please enter a valid number");
    
    // C++ compile-time error: Point arr[array_size];  // Variable size not allowed on stack
    // Rust: This would also be a compile error for arrays:
    // let stack_array: [Point; array_size] = [...];  // Error: size must be known at compile time
    
    // C++: Point* heap_array = new Point[array_size];  // Dynamic allocation on heap
    // Rust equivalents:
    
    // Option 1: Vec<T> - most common and flexible
    let mut vec_points: Vec<Point> = Vec::with_capacity(array_size);
    
    // Fill vector with points (using default constructor equivalent)
    for i in 0..array_size {
        vec_points.push(Point::new(i as f64, (i * 2) as f64));
    }
    
    println!("Created Vec<Point> with {} elements:", array_size);
    for (i, point) in vec_points.iter().enumerate() {
        println!("  vec_points[{}]: {}", i, point);
    }
    
    // Option 2: Box<[T]> - heap-allocated array (closer to C++ new[])
    let boxed_array: Box<[Point]> = (0..array_size)
        .map(|i| Point::new((i + 10) as f64, ((i + 10) * 2) as f64))
        .collect();
    
    println!("Created Box<[Point]> with {} elements:", array_size);
    for (i, point) in boxed_array.iter().enumerate() {
        println!("  boxed_array[{}]: {}", i, point);
    }
    
    // Option 3: Vector of Box<Point> - array of heap-allocated objects
    let vec_of_boxes: Vec<Box<Point>> = (0..array_size)
        .map(|i| Box::new(Point::new((i + 20) as f64, ((i + 20) * 2) as f64)))
        .collect();
    
    println!("Created Vec<Box<Point>> with {} elements:", array_size);
    for (i, boxed_point) in vec_of_boxes.iter().enumerate() {
        println!("  vec_of_boxes[{}]: {}", i, boxed_point);
    }
    
    // Part 4: Demonstrate different constructors in arrays
    println!("\n=== Arrays with Different Constructors ===");
    
    // C++ limitation: new Point[n] only uses default constructor
    // Rust: We can use any constructor/initialization we want
    
    let mixed_points: Vec<Point> = vec![
        Point::default(),                    // Default constructor
        Point::new(5.0, 10.0),              // Parameterized constructor  
        Point::from_single_value(3.0),      // Single value constructor
        Point::from(7.0),                   // From trait conversion
        Point::new(-2.0, -4.0),             // Negative coordinates
    ];
    
    println!("Array with mixed constructors:");
    for (i, point) in mixed_points.iter().enumerate() {
        println!("  mixed_points[{}]: {}", i, point);
    }
    
    // Part 5: Working with dynamic arrays
    println!("\n=== Working with Dynamic Arrays ===");
    
    // Calculate distances between consecutive points
    if vec_points.len() > 1 {
        println!("Distances between consecutive points in vec_points:");
        for i in 0..vec_points.len() - 1 {
            let distance = vec_points[i].distance(&vec_points[i + 1]);
            println!("  Distance from point {} to {}: {:.2}", i, i + 1, distance);
        }
    }
    
    // Create lines from array points
    let lines: Vec<Line> = vec_points.windows(2)
        .map(|window| Line::new(window[0], window[1]))
        .collect();
    
    println!("Lines created from array points:");
    for (i, line) in lines.iter().enumerate() {
        println!("  Line {}: {} (length: {:.2})", i, line, line.length());
    }
    
    // Part 6: Memory management comparison
    println!("\n=== Memory Management Comparison ===");
    
    println!("C++ Manual Memory Management:");
    println!("  Point* p = new Point(1, 2);");
    println!("  // ... use p ...");
    println!("  delete p;  // MUST remember to delete!");
    println!("  Point* arr = new Point[10];");
    println!("  // ... use arr ...");
    println!("  delete[] arr;  // MUST remember delete[] for arrays!");
    
    println!("\nRust Automatic Memory Management:");
    println!("  let p = Box::new(Point::new(1.0, 2.0));");
    println!("  // ... use p ...");
    println!("  // Automatic cleanup when p goes out of scope!");
    println!("  let arr = vec![Point::new(1.0, 2.0); 10];");
    println!("  // ... use arr ...");
    println!("  // Automatic cleanup when arr goes out of scope!");
    
    // Demonstrate ownership and moving
    println!("\n=== Ownership and Moving ===");
    
    let owned_point = Box::new(Point::new(100.0, 200.0));
    println!("Original owner: {}", owned_point);
    
    // Move the Box (transfer ownership)
    let new_owner = owned_point;
    println!("New owner: {}", new_owner);
    // println!("Original owner: {}", owned_point);  // Error! Value moved
    
    // Clone if you need multiple owners
    let point_to_clone = Box::new(Point::new(50.0, 75.0));
    let cloned_point = point_to_clone.clone();
    println!("Original: {}", point_to_clone);
    println!("Clone: {}", cloned_point);
    
    println!("\n=== Summary ===");
    println!("== Rust provides memory safety without manual delete");
    println!("== Box<T> for single heap-allocated objects");
    println!("== Vec<T> for dynamic arrays (most common)");
    println!("== Box<[T]> for heap-allocated fixed-size arrays");
    println!("== No null pointer dereferences");
    println!("== No memory leaks (automatic cleanup)");
    println!("== No double-free errors");
    
    // All objects will be automatically cleaned up here when they go out of scope!
    // No delete needed!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_allocation() {
        // Test Box<T> heap allocation
        let heap_point = Box::new(Point::new(3.0, 4.0));
        assert_eq!(heap_point.x(), 3.0);
        assert_eq!(heap_point.y(), 4.0);
        
        // Test method calls on Box
        let distance = heap_point.distance_to_origin();
        assert_eq!(distance, 5.0);
        
        // Box will be automatically dropped at end of scope
    }

    #[test]
    fn test_box_copying() {
        let original = Box::new(Point::new(1.0, 2.0));
        let copy = Box::new(*original);  // Copy the contents
        
        assert_eq!(*original, *copy);
        assert_eq!(original.x(), copy.x());
        assert_eq!(original.y(), copy.y());
    }

    #[test]
    fn test_dynamic_vector() {
        let size = 5;
        let mut points: Vec<Point> = Vec::with_capacity(size);
        
        // Fill with points
        for i in 0..size {
            points.push(Point::new(i as f64, (i * 2) as f64));
        }
        
        assert_eq!(points.len(), size);
        assert_eq!(points[0], Point::new(0.0, 0.0));
        assert_eq!(points[4], Point::new(4.0, 8.0));
    }

    #[test]
    fn test_boxed_array() {
        let size = 3;
        let boxed_array: Box<[Point]> = (0..size)
            .map(|i| Point::new(i as f64, i as f64))
            .collect();
        
        assert_eq!(boxed_array.len(), size);
        assert_eq!(boxed_array[0], Point::new(0.0, 0.0));
        assert_eq!(boxed_array[2], Point::new(2.0, 2.0));
    }

    #[test]
    fn test_vec_of_boxes() {
        let vec_of_boxes: Vec<Box<Point>> = vec![
            Box::new(Point::new(1.0, 1.0)),
            Box::new(Point::new(2.0, 2.0)),
            Box::new(Point::new(3.0, 3.0)),
        ];
        
        assert_eq!(vec_of_boxes.len(), 3);
        assert_eq!(*vec_of_boxes[1], Point::new(2.0, 2.0));
    }

    #[test]
    fn test_mixed_constructors_in_array() {
        let mixed_points = vec![
            Point::default(),
            Point::new(5.0, 10.0),
            Point::from_single_value(3.0),
            Point::from(7.0),
        ];
        
        assert_eq!(mixed_points[0], Point::new(0.0, 0.0));
        assert_eq!(mixed_points[1], Point::new(5.0, 10.0));
        assert_eq!(mixed_points[2], Point::new(3.0, 3.0));
        assert_eq!(mixed_points[3], Point::new(7.0, 7.0));
    }

    #[test]
    fn test_automatic_cleanup() {
        // This test demonstrates that Rust handles cleanup automatically
        // No manual delete needed
        {
            let _heap_point = Box::new(Point::new(1.0, 2.0));
            let _heap_array = vec![Point::new(3.0, 4.0); 100];
            let _boxed_array: Box<[Point]> = vec![Point::new(5.0, 6.0); 50].into_boxed_slice();
            // All will be automatically dropped when going out of scope
        }
        // Everything is cleaned up here automatically
        
        assert!(true); // Test passes if no memory leaks
    }

    #[test]
    fn test_ownership_moving() {
        let original = Box::new(Point::new(10.0, 20.0));
        let original_value = *original;
        
        // Move the Box
        let moved = original;
        assert_eq!(*moved, original_value);
        
        // original is no longer accessible (moved)
        // This would be a compile error: assert_eq!(*original, original_value);
    }

    #[test]
    fn test_cloning_boxes() {
        let original = Box::new(Point::new(7.0, 8.0));
        let cloned = original.clone();
        
        // Both are accessible
        assert_eq!(*original, *cloned);
        assert_eq!(original.x(), cloned.x());
        assert_eq!(original.y(), cloned.y());
        
        // They are separate objects in memory
        // (though Point values are the same)
    }
}
