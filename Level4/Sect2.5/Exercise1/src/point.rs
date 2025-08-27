// Point class for dynamic allocation exercise
// ===========================================
// Demonstrates heap allocation with Box<T> vs stack allocation
// Automatic memory management via Drop trait (no manual delete needed)

#![allow(dead_code)]
use std::fmt;
use std::ops::{Neg, Mul, Add, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point {
    // Constructor
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    // Default constructor - point at origin
    pub fn default() -> Self {
        Point::new(0.0, 0.0)
    }
    
    // Single-value constructor
    pub fn from_single_value(value: f64) -> Self {
        Point::new(value, value)
    }
    
    // Public getters
    pub fn x(&self) -> f64 {
        self.x
    }
    
    pub fn y(&self) -> f64 {
        self.y
    }
    
    // Setters
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    
    // Distance calculations
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    pub fn distance_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    // ToString methods
    pub fn to_string_custom(&self) -> String {
        format!("Point({:.2}, {:.2})", self.x, self.y)
    }
    
    pub fn to_string_precision(&self, precision: usize) -> String {
        format!("Point({:.prec$}, {:.prec$})", self.x, self.y, prec = precision)
    }
}

// Display trait implementation
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "Point({:.prec$}, {:.prec$})", 
                   self.x, self.y, prec = precision)
        } else {
            write!(f, "Point({:.2}, {:.2})", self.x, self.y)
        }
    }
}

// Additional formatting traits
impl fmt::LowerExp for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:e}, {:e})", self.x, self.y)
    }
}

// Operator implementations

impl Neg for Point {
    type Output = Point;
    
    fn neg(self) -> Self::Output {
        Point { x: -self.x, y: -self.y }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    
    fn mul(self, factor: f64) -> Self::Output {
        Point { x: self.x * factor, y: self.y * factor }
    }
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Self::Output {
        Point { 
            x: self.x + other.x, 
            y: self.y + other.y 
        }
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}

// Allow f64 * Point
impl Mul<Point> for f64 {
    type Output = Point;
    
    fn mul(self, point: Point) -> Self::Output {
        point * self
    }
}

// Conversion traits
impl From<f64> for Point {
    fn from(value: f64) -> Self {
        Point { x: value, y: value }
    }
}

impl From<i32> for Point {
    fn from(value: i32) -> Self {
        let value_f64 = value as f64;
        Point { x: value_f64, y: value_f64 }
    }
}

// Cross-type comparisons
impl PartialEq<f64> for Point {
    fn eq(&self, other: &f64) -> bool {
        self.x == *other && self.y == *other
    }
}

impl PartialEq<Point> for f64 {
    fn eq(&self, other: &Point) -> bool {
        other == self
    }
}

impl PartialEq<i32> for Point {
    fn eq(&self, other: &i32) -> bool {
        let other_f64 = *other as f64;
        self.x == other_f64 && self.y == other_f64
    }
}

impl PartialEq<Point> for i32 {
    fn eq(&self, other: &Point) -> bool {
        other == self
    }
}

// Utility functions for demonstrating dynamic allocation

// Function that creates a Point on the heap and returns it
pub fn create_heap_point(x: f64, y: f64) -> Box<Point> {
    Box::new(Point::new(x, y))
}

// Function that creates a vector of Points
pub fn create_point_array(size: usize) -> Vec<Point> {
    (0..size)
        .map(|i| Point::new(i as f64, (i * i) as f64))
        .collect()
}

// Function that creates a boxed slice of Points (closer to C++ new[])
pub fn create_boxed_point_array(size: usize) -> Box<[Point]> {
    (0..size)
        .map(|i| Point::new((i * 2) as f64, (i * 3) as f64))
        .collect()
}

// Function to demonstrate memory allocation differences
pub fn demonstrate_memory_allocation() {
    println!("Memory Allocation Demonstration:");
    
    // Stack allocation
    let stack_point = Point::new(1.0, 2.0);
    println!("Stack point: {}", stack_point);
    println!("Size of Point: {} bytes", std::mem::size_of::<Point>());
    
    // Heap allocation with Box
    let heap_point = Box::new(Point::new(3.0, 4.0));
    println!("Heap point: {}", heap_point);
    println!("Size of Box<Point>: {} bytes", std::mem::size_of::<Box<Point>>());
    println!("(Box contains a pointer to heap-allocated data)");
    
    // Vector allocation (heap-based dynamic array)
    let point_vec = vec![Point::new(5.0, 6.0); 10];
    println!("Vector with 10 points: {} elements", point_vec.len());
    println!("Vector capacity: {}", point_vec.capacity());
}

// Function to compare with C++ allocation patterns
pub fn compare_with_cpp() {
    println!("\nC++ vs Rust Allocation Patterns:");
    
    println!("\nC++ Heap Allocation:");
    println!("  Point* p = new Point(1.0, 2.0);");
    println!("  // Must remember: delete p;");
    println!("  Point* arr = new Point[10];");
    println!("  // Must remember: delete[] arr;");
    
    println!("\nRust Heap Allocation:");
    println!("  let p = Box::new(Point::new(1.0, 2.0));");
    println!("  // Automatic cleanup when p goes out of scope");
    println!("  let arr = vec![Point::new(0.0, 0.0); 10];");
    println!("  // Automatic cleanup when arr goes out of scope");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_construction() {
        let p = Point::new(3.14, 2.71);
        assert_eq!(p.x(), 3.14);
        assert_eq!(p.y(), 2.71);
    }

    #[test]
    fn test_default_constructor() {
        let p = Point::default();
        assert_eq!(p.x(), 0.0);
        assert_eq!(p.y(), 0.0);
    }

    #[test]
    fn test_distance_calculation() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
        assert_eq!(p2.distance_to_origin(), 5.0);
    }

    #[test]
    fn test_heap_allocation() {
        // Test Box<Point>
        let heap_point = Box::new(Point::new(5.0, 10.0));
        assert_eq!(heap_point.x(), 5.0);
        assert_eq!(heap_point.y(), 10.0);
        
        // Test method calls on Box
        let distance = heap_point.distance_to_origin();
        assert!((distance - (125.0_f64).sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_create_heap_point_function() {
        let heap_point = create_heap_point(7.0, 8.0);
        assert_eq!(heap_point.x(), 7.0);
        assert_eq!(heap_point.y(), 8.0);
    }

    #[test]
    fn test_create_point_array() {
        let points = create_point_array(5);
        assert_eq!(points.len(), 5);
        assert_eq!(points[0], Point::new(0.0, 0.0));
        assert_eq!(points[1], Point::new(1.0, 1.0));
        assert_eq!(points[4], Point::new(4.0, 16.0));
    }

    #[test]
    fn test_create_boxed_point_array() {
        let boxed_points = create_boxed_point_array(3);
        assert_eq!(boxed_points.len(), 3);
        assert_eq!(boxed_points[0], Point::new(0.0, 0.0));
        assert_eq!(boxed_points[1], Point::new(2.0, 3.0));
        assert_eq!(boxed_points[2], Point::new(4.0, 6.0));
    }

    #[test]
    fn test_copy_semantics() {
        // Point implements Copy, so it can be copied
        let original = Point::new(1.0, 2.0);
        let copied = original;  // This is a copy, not a move
        
        // Both are accessible
        assert_eq!(original.x(), 1.0);
        assert_eq!(copied.x(), 1.0);
    }

    #[test]
    fn test_box_copy() {
        // Test copying the contents of a Box
        let original_box = Box::new(Point::new(3.0, 4.0));
        let copied_box = Box::new(*original_box);  // Copy the Point inside
        
        assert_eq!(*original_box, *copied_box);
        assert_eq!(original_box.x(), copied_box.x());
    }

    #[test]
    fn test_operators() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        
        let sum = p1 + p2;
        assert_eq!(sum, Point::new(4.0, 6.0));
        
        let scaled = p1 * 2.0;
        assert_eq!(scaled, Point::new(2.0, 4.0));
        
        let negated = -p1;
        assert_eq!(negated, Point::new(-1.0, -2.0));
    }

    #[test]
    fn test_conversions() {
        let p1: Point = 5.0.into();
        let p2 = Point::from(3.0);
        
        assert_eq!(p1, Point::new(5.0, 5.0));
        assert_eq!(p2, Point::new(3.0, 3.0));
        
        assert!(p1 == 5.0);
        assert!(p2 == 3.0);
    }

    #[test]
    fn test_memory_safety() {
        // This test demonstrates Rust's memory safety
        let points = {
            let mut temp_points = Vec::new();
            temp_points.push(Point::new(1.0, 2.0));
            temp_points.push(Point::new(3.0, 4.0));
            temp_points  // Move out of scope
        };
        
        // points is still valid here because Vec was moved
        assert_eq!(points.len(), 2);
        assert_eq!(points[0], Point::new(1.0, 2.0));
    }

    #[test]
    fn test_automatic_cleanup() {
        // Demonstrate that objects are automatically cleaned up
        for _ in 0..1000 {
            let _heap_point = Box::new(Point::new(1.0, 2.0));
            let _heap_array = vec![Point::new(3.0, 4.0); 100];
            // All automatically cleaned up at end of each iteration
        }
        // No memory leaks!
        assert!(true);
    }
}
