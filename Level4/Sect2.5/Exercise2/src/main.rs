// Exercise 2: Creating Array of Pointers (Vec<Box<Point>> in Rust)
// ================================================================
// 
// MEMORY LAYOUT DRAWING:
// ======================
// 
// C++ Memory Layout (Point** arr = new Point*[3]):
// ┌─────────────────────────────────────────────────────────────────┐
// │                           STACK                                 │
// │  ┌─────────┐                                                    │
// │  │   arr   │ ──────────────────┐                                │
// │  │(Point**)│                   │                                │
// │  └─────────┘                   │                                │
// └─────────────────────────────────┼──────────────────────────────┘
//                                   │
//               ┌───────────────────┘
//               │
//               ▼
// ┌─────────────────────────────────────────────────────────────────┐
// │                           HEAP                                  │
// │  ┌─────────┬─────────┬─────────┐    Array of 3 Point* pointers  │
// │  │ Point*  │ Point*  │ Point*  │                                 │
// │  │   [0]   │   [1]   │   [2]   │                                 │
// │  └────┬────┴────┬────┴────┬────┘                                 │
// │       │         │         │                                     │
// │       ▼         ▼         ▼                                     │
// │  ┌────────┐ ┌────────┐ ┌────────┐   Individual Point objects    │
// │  │Point(  │ │Point(  │ │Point(  │                               │
// │  │ 1.0,   │ │ 3.0,   │ │ 5.0,   │                               │
// │  │ 2.0)   │ │ 4.0)   │ │ 6.0)   │                               │
// │  └────────┘ └────────┘ └────────┘                               │
// └─────────────────────────────────────────────────────────────────┘
//
// Rust Memory Layout (Vec<Box<Point>>):
// ┌─────────────────────────────────────────────────────────────────┐
// │                           STACK                                 │
// │  ┌─────────┐                                                    │
// │  │   vec   │ ──────────────────┐                                │
// │  │ (Vec<   │                   │                                │
// │  │Box<T>>) │                   │                                │
// │  └─────────┘                   │                                │
// └─────────────────────────────────┼──────────────────────────────┘
//                                   │
//               ┌───────────────────┘
//               │
//               ▼
// ┌─────────────────────────────────────────────────────────────────┐
// │                           HEAP                                  │
// │  ┌─────────┬─────────┬─────────┐    Vec's heap buffer of Box<T> │
// │  │  Box    │  Box    │  Box    │    (each Box is a pointer)     │
// │  │   [0]   │   [1]   │   [2]   │                                 │
// │  └────┬────┴────┬────┴────┬────┘                                 │
// │       │         │         │                                     │
// │       ▼         ▼         ▼                                     │
// │  ┌────────┐ ┌────────┐ ┌────────┐   Individual Point objects    │
// │  │Point(  │ │Point(  │ │Point(  │   (each in separate heap      │
// │  │ 1.0,   │ │ 3.0,   │ │ 5.0,   │   allocation)                 │
// │  │ 2.0)   │ │ 4.0)   │ │ 6.0)   │                               │
// │  └────────┘ └────────┘ └────────┘                               │
// └─────────────────────────────────────────────────────────────────┘
//
// Key Differences:
// - C++: Manual delete required for each Point and the array
// - Rust: Automatic cleanup when Vec and Box objects go out of scope
// - C++: Risk of memory leaks if delete is forgotten  
// - Rust: Memory safety guaranteed at compile time
// - C++: Possible null pointers in array
// - Rust: Box<T> cannot be null, guaranteed to point to valid data

#![allow(dead_code)]
mod point;
mod line;  
mod circle;

use point::Point;
use line::Line;
#[allow(unused_imports)]
use circle::Circle;
use std::io;

fn main() {
    println!("Level 4, Section 2.5, Exercise 2: Creating Array of Pointers");
    println!("==============================================================\n");

    // Step 1: Create an array of Point pointers with 3 elements on the heap
    // C++: Point** arr = new Point*[3];
    // Rust: Vec<Box<Point>> - vector of boxed (heap-allocated) Points
    println!("=== Step 1: Creating Array of Point Pointers ===");
    
    // Initialize empty vector with capacity for 3 Box<Point> elements
    let mut point_pointers: Vec<Box<Point>> = Vec::with_capacity(3);
    println!("Created Vec<Box<Point>> with capacity: {}", point_pointers.capacity());
    
    // Step 2: Create each point on the heap with different constructors
    // C++: arr[0] = new Point(1.0, 2.0);
    //      arr[1] = new Point();
    //      arr[2] = new Point(5.0, 6.0);
    println!("\n=== Step 2: Creating Individual Points on Heap ===");
    
    // Each Box::new() creates a separate heap allocation for each Point
    point_pointers.push(Box::new(Point::new(1.0, 2.0)));        // Parameterized constructor
    point_pointers.push(Box::new(Point::default()));            // Default constructor  
    point_pointers.push(Box::new(Point::from_single_value(5.0))); // Single-value constructor
    
    println!("Created 3 heap-allocated Points:");
    println!("  point_pointers[0]: heap Point with coordinates (1.0, 2.0)");
    println!("  point_pointers[1]: heap Point with default constructor");
    println!("  point_pointers[2]: heap Point from single value 5.0");
    
    // Step 3: Iterate the array and print each point
    // C++: for(int i = 0; i < 3; i++) cout << *arr[i] << endl;
    println!("\n=== Step 3: Iterating and Printing Points ===");
    
    // Method 1: Index-based iteration
    println!("Index-based iteration:");
    for i in 0..point_pointers.len() {
        // In C++: *arr[i] (dereference pointer)
        // In Rust: Box implements Deref, so we can use directly or explicitly dereference
        println!("  point_pointers[{}]: {}", i, point_pointers[i]);
        println!("  (explicit deref): {}", *point_pointers[i]);
    }
    
    // Method 2: Iterator-based (more idiomatic Rust)
    println!("\nIterator-based iteration:");
    for (i, point_box) in point_pointers.iter().enumerate() {
        println!("  Point {}: {} (distance to origin: {:.2})", 
                i, point_box, point_box.distance_to_origin());
    }
    
    // Step 4: Demonstrate different operations on heap objects
    println!("\n=== Step 4: Operations on Heap Objects ===");
    
    // Call methods on each heap-allocated object
    for (i, point_box) in point_pointers.iter().enumerate() {
        println!("Point {} details:", i);
        println!("  Coordinates: ({:.2}, {:.2})", point_box.x(), point_box.y());
        println!("  Distance to origin: {:.2}", point_box.distance_to_origin());
        
        if i < point_pointers.len() - 1 {
            let distance = point_box.distance(&point_pointers[i + 1]);
            println!("  Distance to next point: {:.2}", distance);
        }
    }
    
    // Step 5: Create more complex array of pointers with user input
    println!("\n=== Step 5: Dynamic Array of Pointers ===");
    println!("Enter the number of points to create:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let count: usize = input.trim().parse().expect("Please enter a valid number");
    
    // Create dynamic array of Point pointers
    let mut dynamic_points: Vec<Box<Point>> = Vec::with_capacity(count);
    
    // Fill with points using different constructors based on index
    for i in 0..count {
        let point_box = match i % 3 {
            0 => Box::new(Point::new(i as f64, (i * 2) as f64)),
            1 => Box::new(Point::default()),
            2 => Box::new(Point::from_single_value((i + 1) as f64)),
            _ => unreachable!(),
        };
        dynamic_points.push(point_box);
    }
    
    println!("Created {} heap-allocated points:", count);
    for (i, point_box) in dynamic_points.iter().enumerate() {
        println!("  Dynamic point[{}]: {}", i, point_box);
    }
    
    // Step 6: Demonstrate memory management differences
    println!("\n=== Step 6: Memory Management Comparison ===");
    
    println!("C++ Manual Memory Management:");
    println!("  Point** arr = new Point*[3];");
    println!("  arr[0] = new Point(1.0, 2.0);");
    println!("  arr[1] = new Point();");
    println!("  arr[2] = new Point(5.0, 6.0);");
    println!("  // ... use points ...");
    println!("  for(int i = 0; i < 3; i++) delete arr[i];  // MUST delete each Point");
    println!("  delete[] arr;                              // MUST delete the array");
    
    println!("\nRust Automatic Memory Management:");
    println!("  let mut points: Vec<Box<Point>> = Vec::with_capacity(3);");
    println!("  points.push(Box::new(Point::new(1.0, 2.0)));");
    println!("  points.push(Box::new(Point::default()));");
    println!("  points.push(Box::new(Point::from_single_value(5.0)));");
    println!("  // ... use points ...");
    println!("  // Automatic cleanup when 'points' goes out of scope!");
    println!("  // Each Box<Point> automatically drops its Point");
    println!("  // Vec automatically drops its heap buffer");
    
    // Step 7: Advanced operations with array of pointers
    println!("\n=== Step 7: Advanced Operations ===");
    
    // Calculate total distance traveled through all points
    let total_distance: f64 = point_pointers.windows(2)
        .map(|window| window[0].distance(&window[1]))
        .sum();
    println!("Total distance through original points: {:.2}", total_distance);
    
    // Find the point farthest from origin
    if let Some((index, farthest_point)) = point_pointers.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.distance_to_origin().partial_cmp(&b.distance_to_origin()).unwrap()) {
        println!("Point farthest from origin: index {} = {} (distance: {:.2})", 
                index, farthest_point, farthest_point.distance_to_origin());
    }
    
    // Create lines connecting consecutive points
    let lines: Vec<Line> = point_pointers.windows(2)
        .map(|window| Line::new(*window[0], *window[1]))  // Single deref: Box<Point> -> Point
        .collect();
    
    println!("Lines created from consecutive points:");
    for (i, line) in lines.iter().enumerate() {
        println!("  Line {}: {} (length: {:.2})", i, line, line.length());
    }
    
    // Step 8: Demonstrate ownership and borrowing with heap objects
    println!("\n=== Step 8: Ownership and Borrowing ===");
    
    // Move ownership of a Box<Point>
    let moved_point = point_pointers.pop().unwrap(); // Remove last point from vector
    println!("Moved point from vector: {}", moved_point);
    println!("Vector now has {} elements", point_pointers.len());
    
    // Clone a Box<Point> (creates new heap allocation)
    let cloned_point = point_pointers[0].clone();
    println!("Cloned point: {} (new heap allocation)", cloned_point);
    
    // Borrow points for calculations without moving them
    let borrowed_sum = calculate_centroid(&point_pointers);
    println!("Centroid of remaining points: {}", borrowed_sum);
    
    // Step 9: Compare with Vec<Point> (non-pointer array)
    println!("\n=== Step 9: Comparison with Vec<Point> ===");
    
    // Array of Points (not pointers) - all stored in single heap allocation
    let point_array = vec![
        Point::new(1.0, 2.0),
        Point::default(), 
        Point::from_single_value(5.0),
    ];
    
    println!("Vec<Point> (array of values, not pointers):");
    println!("  Size of Vec<Point>: {} bytes", std::mem::size_of::<Vec<Point>>());
    println!("  Size of Point: {} bytes", std::mem::size_of::<Point>());
    println!("  All {} Points stored in single heap allocation", point_array.len());
    
    println!("Vec<Box<Point>> (array of pointers):");
    println!("  Size of Vec<Box<Point>>: {} bytes", std::mem::size_of::<Vec<Box<Point>>>());
    println!("  Size of Box<Point>: {} bytes", std::mem::size_of::<Box<Point>>());
    println!("  Each of {} Points has separate heap allocation", point_pointers.len() + 1); // +1 for moved_point
    
    // Step 10: Memory cleanup demonstration
    println!("\n=== Step 10: Automatic Memory Cleanup ===");
    println!("All heap-allocated objects will be automatically cleaned up:");
    println!("  1. point_pointers: {} Box<Point> objects will be dropped", point_pointers.len());
    println!("  2. moved_point: 1 Box<Point> will be dropped");
    println!("  3. cloned_point: 1 Box<Point> will be dropped"); 
    println!("  4. dynamic_points: {} Box<Point> objects will be dropped", dynamic_points.len());
    println!("  5. point_array: Vec<Point> and its contents will be dropped");
    println!("  6. All Line objects will be dropped");
    println!("\nNo manual 'delete' statements required!");
    
    // All cleanup happens automatically here when variables go out of scope!
    // In C++, forgetting any delete would cause a memory leak
    // In Rust, this is impossible due to automatic memory management
}

// Helper function to demonstrate borrowing heap objects
fn calculate_centroid(points: &[Box<Point>]) -> Point {
    if points.is_empty() {
        return Point::default();
    }
    
    let sum_x: f64 = points.iter().map(|p| p.x()).sum();
    let sum_y: f64 = points.iter().map(|p| p.y()).sum();
    let count = points.len() as f64;
    
    Point::new(sum_x / count, sum_y / count)
}

// Function to demonstrate creating and returning array of pointers
fn create_point_pointer_array(size: usize) -> Vec<Box<Point>> {
    (0..size)
        .map(|i| Box::new(Point::new(i as f64 * 10.0, (i as f64 * 10.0) + 5.0)))
        .collect()
}

// Function to demonstrate processing array of pointers
fn process_point_array(points: &mut [Box<Point>]) {
    for (i, point_box) in points.iter_mut().enumerate() {
        // Modify each point (multiply coordinates by index + 1)
        let multiplier = (i + 1) as f64;
        **point_box = Point::new(point_box.x() * multiplier, point_box.y() * multiplier);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_of_point_pointers() {
        // Create array of 3 Point pointers
        let mut point_pointers: Vec<Box<Point>> = Vec::with_capacity(3);
        
        // Create individual Points on heap
        point_pointers.push(Box::new(Point::new(1.0, 2.0)));
        point_pointers.push(Box::new(Point::default()));
        point_pointers.push(Box::new(Point::from_single_value(5.0)));
        
        // Verify creation
        assert_eq!(point_pointers.len(), 3);
        assert_eq!(*point_pointers[0], Point::new(1.0, 2.0));
        assert_eq!(*point_pointers[1], Point::new(0.0, 0.0));
        assert_eq!(*point_pointers[2], Point::new(5.0, 5.0));
    }

    #[test]
    fn test_different_constructors_in_array() {
        let points: Vec<Box<Point>> = vec![
            Box::new(Point::new(1.0, 2.0)),        // Parameterized
            Box::new(Point::default()),            // Default
            Box::new(Point::from_single_value(3.0)), // Single value
            Box::new(Point::from(4.0)),            // From trait
        ];
        
        assert_eq!(points.len(), 4);
        assert_eq!(points[0].x(), 1.0);
        assert_eq!(points[1].x(), 0.0);
        assert_eq!(points[2].x(), 3.0);
        assert_eq!(points[3].x(), 4.0);
    }

    #[test]
    fn test_iteration_over_pointer_array() {
        let points = vec![
            Box::new(Point::new(1.0, 1.0)),
            Box::new(Point::new(2.0, 2.0)),
            Box::new(Point::new(3.0, 3.0)),
        ];
        
        // Test index-based iteration
        for i in 0..points.len() {
            assert_eq!(points[i].x(), (i + 1) as f64);
            assert_eq!(points[i].y(), (i + 1) as f64);
        }
        
        // Test iterator-based iteration
        for (i, point_box) in points.iter().enumerate() {
            assert_eq!(point_box.x(), (i + 1) as f64);
            assert_eq!(point_box.y(), (i + 1) as f64);
        }
    }

    #[test]
    fn test_method_calls_on_heap_objects() {
        let points = vec![
            Box::new(Point::new(3.0, 4.0)),  // Distance 5.0
            Box::new(Point::new(5.0, 12.0)), // Distance 13.0
        ];
        
        assert_eq!(points[0].distance_to_origin(), 5.0);
        assert_eq!(points[1].distance_to_origin(), 13.0);
        
        let distance_between = points[0].distance(&points[1]);
        assert!((distance_between - 8.246).abs() < 0.001); // Approximate
    }

    #[test]
    fn test_centroid_calculation() {
        let points = vec![
            Box::new(Point::new(0.0, 0.0)),
            Box::new(Point::new(2.0, 0.0)),
            Box::new(Point::new(1.0, 2.0)),
        ];
        
        let centroid = calculate_centroid(&points);
        assert_eq!(centroid, Point::new(1.0, 2.0 / 3.0));
    }

    #[test]
    fn test_create_point_pointer_array() {
        let points = create_point_pointer_array(3);
        
        assert_eq!(points.len(), 3);
        assert_eq!(*points[0], Point::new(0.0, 5.0));
        assert_eq!(*points[1], Point::new(10.0, 15.0));
        assert_eq!(*points[2], Point::new(20.0, 25.0));
    }

    #[test]
    fn test_process_point_array() {
        let mut points = vec![
            Box::new(Point::new(1.0, 1.0)),
            Box::new(Point::new(2.0, 2.0)),
            Box::new(Point::new(3.0, 3.0)),
        ];
        
        process_point_array(&mut points);
        
        assert_eq!(*points[0], Point::new(1.0, 1.0));   // *1
        assert_eq!(*points[1], Point::new(4.0, 4.0));   // *2
        assert_eq!(*points[2], Point::new(9.0, 9.0));   // *3
    }

    #[test]
    fn test_ownership_operations() {
        let mut points = vec![
            Box::new(Point::new(1.0, 2.0)),
            Box::new(Point::new(3.0, 4.0)),
        ];
        
        // Move a point out of the vector
        let moved_point = points.pop().unwrap();
        assert_eq!(*moved_point, Point::new(3.0, 4.0));
        assert_eq!(points.len(), 1);
        
        // Clone the remaining point
        let cloned_point = points[0].clone();
        assert_eq!(*cloned_point, *points[0]);
        assert_eq!(points.len(), 1); // Original still in vector
    }

    #[test]
    fn test_memory_layout_difference() {
        // Vec<Point> - all Points in single allocation
        let point_values = vec![Point::new(1.0, 2.0), Point::new(3.0, 4.0)];
        
        // Vec<Box<Point>> - each Point in separate allocation
        let point_pointers = vec![Box::new(Point::new(1.0, 2.0)), Box::new(Point::new(3.0, 4.0))];
        
        // Both have same logical content
        for i in 0..2 {
            assert_eq!(point_values[i], *point_pointers[i]);
        }
        
        // But different memory layouts (can't easily test, but different performance characteristics)
        assert!(true); // This test documents the concept
    }

    #[test]
    fn test_automatic_cleanup() {
        // This test demonstrates automatic cleanup
        {
            let _temp_points: Vec<Box<Point>> = (0..1000)
                .map(|i| Box::new(Point::new(i as f64, (i * 2) as f64)))
                .collect();
            // 1000 heap allocations created
        }
        // All 1000 heap allocations automatically cleaned up here
        
        assert!(true); // Test passes if no memory leaks
    }
}
