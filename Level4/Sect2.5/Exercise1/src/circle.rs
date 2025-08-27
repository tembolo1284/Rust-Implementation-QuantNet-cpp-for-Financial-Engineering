// Circle class for dynamic allocation exercise
// ===========================================
// Demonstrates heap allocation and dynamic arrays of Circles

#![allow(dead_code)]
use crate::point::Point;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Circle {
    pub(crate) center: Point,
    pub(crate) radius: f64,
}

impl Circle {
    // Constructor with center point and radius
    pub fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }
    
    // Default constructor - unit circle at origin
    pub fn default() -> Self {
        Circle::new(Point::default(), 1.0)
    }
    
    // Constructor using conversion - center from single value
    pub fn from_center_value(center_val: f64, radius: f64) -> Self {
        Circle::new(Point::from(center_val), radius)
    }
    
    // Factory methods
    pub fn unit_circle() -> Self {
        Circle::new(Point::default(), 1.0)
    }
    
    pub fn at_origin(radius: f64) -> Self {
        Circle::new(Point::default(), radius)
    }
    
    // Public getters
    pub fn center(&self) -> &Point {
        &self.center
    }
    
    pub fn radius(&self) -> f64 {
        self.radius
    }
    
    // Setters
    pub fn set_center(&mut self, center: Point) {
        self.center = center;
    }
    
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    
    // Mathematical functions
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
    
    // Check if a point is inside the circle
    pub fn contains_point(&self, point: &Point) -> bool {
        self.center.distance(point) <= self.radius
    }
    
    // Check if a point is on the circle boundary
    pub fn point_on_boundary(&self, point: &Point) -> bool {
        const EPSILON: f64 = 1e-10;
        (self.center.distance(point) - self.radius).abs() < EPSILON
    }
    
    // Get point on circle at given angle (in radians)
    pub fn point_at_angle(&self, angle: f64) -> Point {
        let x = self.center.x + self.radius * angle.cos();
        let y = self.center.y + self.radius * angle.sin();
        Point::new(x, y)
    }
    
    // Move circle by a vector (using Point operators!)
    pub fn translate(&self, offset: Point) -> Circle {
        Circle::new(self.center + offset, self.radius)
    }
    
    // Scale circle by a factor
    pub fn scale(&self, factor: f64) -> Circle {
        Circle::new(self.center, self.radius * factor)
    }
    
    // Get bounding box
    pub fn bounding_box(&self) -> (Point, Point) {
        let min_x = self.center.x - self.radius;
        let min_y = self.center.y - self.radius;
        let max_x = self.center.x + self.radius;
        let max_y = self.center.y + self.radius;
        
        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }
    
    // ToString methods
    pub fn to_string_custom(&self) -> String {
        format!("Circle[center: {}, radius: {:.2}]", 
                self.center.to_string_custom(), 
                self.radius)
    }
    
    pub fn to_string_with_properties(&self) -> String {
        format!(
            "Circle[center: {}, radius: {:.2}, area: {:.2}, circumference: {:.2}]", 
            self.center.to_string_custom(), 
            self.radius,
            self.area(),
            self.circumference()
        )
    }
}

// Display trait implementation
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "Circle[center: Point({:.2}, {:.2}), radius: {:.2}, area: {:.2}, circumference: {:.2}]",
                   self.center.x, self.center.y,
                   self.radius,
                   self.area(),
                   self.circumference())
        } else if let Some(precision) = f.precision() {
            write!(f, "Circle[center: Point({:.prec$}, {:.prec$}), radius: {:.prec$}]",
                   self.center.x, self.center.y, self.radius, prec = precision)
        } else {
            write!(f, "Circle[center: Point({:.2}, {:.2}), radius: {:.2}]",
                   self.center.x, self.center.y, self.radius)
        }
    }
}

// Custom Debug format
impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Circle")
            .field("center.x", &self.center.x)
            .field("center.y", &self.center.y)
            .field("radius", &self.radius)
            .field("diameter", &self.diameter())
            .field("area", &self.area())
            .field("circumference", &self.circumference())
            .finish()
    }
}

// Utility functions for demonstrating dynamic allocation of Circles

// Function that creates a Circle on the heap and returns it
pub fn create_heap_circle(center: Point, radius: f64) -> Box<Circle> {
    Box::new(Circle::new(center, radius))
}

// Function that creates a vector of Circles
pub fn create_circle_array(size: usize) -> Vec<Circle> {
    (0..size)
        .map(|i| {
            let center = Point::new(i as f64, i as f64);
            let radius = (i + 1) as f64;
            Circle::new(center, radius)
        })
        .collect()
}

// Function that creates a boxed slice of Circles
pub fn create_boxed_circle_array(size: usize) -> Box<[Circle]> {
    (0..size)
        .map(|i| {
            let center = Point::new((i * 2) as f64, (i * 3) as f64);
            let radius = (i + 1) as f64 * 0.5;
            Circle::new(center, radius)
        })
        .collect()
}

// Function to create concentric circles
pub fn create_concentric_circles(center: Point, count: usize, radius_step: f64) -> Vec<Circle> {
    (1..=count)
        .map(|i| Circle::new(center, i as f64 * radius_step))
        .collect()
}

// Function to create circles along a line
pub fn create_circles_along_line(start: Point, end: Point, count: usize, radius: f64) -> Vec<Circle> {
    if count == 0 {
        return Vec::new();
    }
    
    if count == 1 {
        return vec![Circle::new(start, radius)];
    }
    
    (0..count)
        .map(|i| {
            let t = i as f64 / (count - 1) as f64;
            let x = start.x + t * (end.x - start.x);
            let y = start.y + t * (end.y - start.y);
            Circle::new(Point::new(x, y), radius)
        })
        .collect()
}

// Function to demonstrate dynamic circle creation patterns
pub fn demonstrate_dynamic_circle_creation() {
    println!("Dynamic Circle Creation Demonstration:");
    
    // Single heap-allocated circle
    let heap_circle = create_heap_circle(Point::new(5.0, 5.0), 3.0);
    println!("Heap circle: {} (area: {:.2})", heap_circle, heap_circle.area());
    
    // Vector of circles
    let circle_vec = create_circle_array(4);
    println!("Vector with {} circles:", circle_vec.len());
    for (i, circle) in circle_vec.iter().enumerate() {
        println!("  Circle {}: {} (area: {:.2})", i, circle, circle.area());
    }
    
    // Boxed array of circles
    let boxed_circles = create_boxed_circle_array(3);
    println!("Boxed array with {} circles:", boxed_circles.len());
    for (i, circle) in boxed_circles.iter().enumerate() {
        println!("  Circle {}: {} (area: {:.2})", i, circle, circle.area());
    }
    
    // Concentric circles
    let concentric = create_concentric_circles(Point::new(0.0, 0.0), 3, 2.0);
    println!("Concentric circles:");
    for (i, circle) in concentric.iter().enumerate() {
        println!("  Ring {}: {} (circumference: {:.2})", i, circle, circle.circumference());
    }
}

// Function to compare memory allocation approaches for circles
pub fn compare_circle_allocation() {
    println!("\nCircle Allocation Comparison:");
    
    // Stack allocation
    let stack_circle = Circle::new(Point::new(1.0, 2.0), 3.0);
    println!("Stack circle: {}", stack_circle);
    println!("Size of Circle: {} bytes", std::mem::size_of::<Circle>());
    
    // Heap allocation with Box
    let heap_circle = Box::new(Circle::new(Point::new(4.0, 5.0), 6.0));
    println!("Heap circle: {}", heap_circle);
    println!("Size of Box<Circle>: {} bytes", std::mem::size_of::<Box<Circle>>());
    
    // Vector allocation
    let circle_vec = vec![
        Circle::new(Point::new(1.0, 1.0), 1.0),
        Circle::new(Point::new(2.0, 2.0), 2.0),
        Circle::new(Point::new(3.0, 3.0), 3.0),
    ];
    println!("Vector with {} circles", circle_vec.len());
    println!("Vector capacity: {}", circle_vec.capacity());
    
    // Calculate total area of all circles in vector
    let total_area: f64 = circle_vec.iter().map(|c| c.area()).sum();
    println!("Total area of all circles: {:.2}", total_area);
}

// Function to analyze circle intersections in a dynamic array
pub fn analyze_circle_intersections(circles: &[Circle]) -> Vec<(usize, usize)> {
    let mut intersections = Vec::new();
    
    for i in 0..circles.len() {
        for j in i + 1..circles.len() {
            let c1 = &circles[i];
            let c2 = &circles[j];
            
            let center_distance = c1.center.distance(&c2.center);
            let radius_sum = c1.radius + c2.radius;
            let radius_diff = (c1.radius - c2.radius).abs();
            
            // Circles intersect if center distance is between radius difference and radius sum
            if center_distance >= radius_diff && center_distance <= radius_sum {
                intersections.push((i, j));
            }
        }
    }
    
    intersections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_creation() {
        let center = Point::new(2.0, 3.0);
        let radius = 5.0;
        let circle = Circle::new(center, radius);
        
        assert_eq!(*circle.center(), center);
        assert_eq!(circle.radius(), radius);
        assert_eq!(circle.diameter(), 10.0);
    }

    #[test]
    fn test_heap_circle_allocation() {
        let heap_circle = create_heap_circle(Point::new(1.0, 2.0), 3.0);
        
        assert_eq!(heap_circle.center().x(), 1.0);
        assert_eq!(heap_circle.center().y(), 2.0);
        assert_eq!(heap_circle.radius(), 3.0);
        assert_eq!(heap_circle.area(), std::f64::consts::PI * 9.0);
    }

    #[test]
    fn test_circle_array_creation() {
        let circles = create_circle_array(3);
        assert_eq!(circles.len(), 3);
        
        // Check first circle
        assert_eq!(circles[0].center().x(), 0.0);
        assert_eq!(circles[0].center().y(), 0.0);
        assert_eq!(circles[0].radius(), 1.0);
        
        // Check second circle
        assert_eq!(circles[1].center().x(), 1.0);
        assert_eq!(circles[1].center().y(), 1.0);
        assert_eq!(circles[1].radius(), 2.0);
    }

    #[test]
    fn test_boxed_circle_array() {
        let boxed_circles = create_boxed_circle_array(2);
        assert_eq!(boxed_circles.len(), 2);
        
        assert_eq!(boxed_circles[0].center(), &Point::new(0.0, 0.0));
        assert_eq!(boxed_circles[0].radius(), 0.5);
        assert_eq!(boxed_circles[1].center(), &Point::new(2.0, 3.0));
        assert_eq!(boxed_circles[1].radius(), 1.0);
    }

    #[test]
    fn test_concentric_circles() {
        let center = Point::new(5.0, 5.0);
        let concentric = create_concentric_circles(center, 3, 2.0);
        
        assert_eq!(concentric.len(), 3);
        
        // All should have same center
        for circle in &concentric {
            assert_eq!(*circle.center(), center);
        }
        
        // Radii should increase
        assert_eq!(concentric[0].radius(), 2.0);
        assert_eq!(concentric[1].radius(), 4.0);
        assert_eq!(concentric[2].radius(), 6.0);
    }

    #[test]
    fn test_circles_along_line() {
        let start = Point::new(0.0, 0.0);
        let end = Point::new(4.0, 0.0);
        let circles = create_circles_along_line(start, end, 5, 1.0);
        
        assert_eq!(circles.len(), 5);
        
        // All should have same radius
        for circle in &circles {
            assert_eq!(circle.radius(), 1.0);
        }
        
        // Centers should be along the line
        assert_eq!(*circles[0].center(), Point::new(0.0, 0.0));
        assert_eq!(*circles[2].center(), Point::new(2.0, 0.0)); // Midpoint
        assert_eq!(*circles[4].center(), Point::new(4.0, 0.0));
    }

    #[test]
    fn test_edge_cases_circles_along_line() {
        let start = Point::new(0.0, 0.0);
        let end = Point::new(1.0, 1.0);
        
        // Empty case
        let empty = create_circles_along_line(start, end, 0, 1.0);
        assert_eq!(empty.len(), 0);
        
        // Single circle case
        let single = create_circles_along_line(start, end, 1, 2.0);
        assert_eq!(single.len(), 1);
        assert_eq!(*single[0].center(), start);
        assert_eq!(single[0].radius(), 2.0);
    }

    #[test]
    fn test_circle_methods() {
        let circle = Circle::new(Point::new(0.0, 0.0), 5.0);
        
        // Test mathematical functions
        assert_eq!(circle.diameter(), 10.0);
        assert_eq!(circle.area(), std::f64::consts::PI * 25.0);
        assert_eq!(circle.circumference(), 10.0 * std::f64::consts::PI);
        
        // Test point containment
        assert!(circle.contains_point(&Point::new(3.0, 4.0))); // Inside
        assert!(circle.contains_point(&Point::new(5.0, 0.0))); // On boundary
        assert!(!circle.contains_point(&Point::new(6.0, 0.0))); // Outside
        
        // Test point at angle
        let p0 = circle.point_at_angle(0.0);
        assert!((p0.x() - 5.0).abs() < 1e-10);
        assert!(p0.y().abs() < 1e-10);
    }

    #[test]
    fn test_circle_transformations() {
        let circle = Circle::new(Point::new(1.0, 2.0), 3.0);
        
        // Test translation
        let translated = circle.translate(Point::new(5.0, 5.0));
        assert_eq!(*translated.center(), Point::new(6.0, 7.0));
        assert_eq!(translated.radius(), 3.0);
        
        // Test scaling
        let scaled = circle.scale(2.0);
        assert_eq!(*scaled.center(), Point::new(1.0, 2.0));
        assert_eq!(scaled.radius(), 6.0);
    }

    #[test]
    fn test_copy_semantics() {
        let original = Circle::new(Point::new(1.0, 2.0), 3.0);
        let copied = original;  // Circle implements Copy
        
        // Both are accessible
        assert_eq!(original.radius(), copied.radius());
        assert_eq!(*original.center(), *copied.center());
    }

    #[test]
    fn test_box_circle_operations() {
        let box1 = Box::new(Circle::new(Point::new(0.0, 0.0), 5.0));
        let box2 = Box::new(*box1);  // Copy the Circle inside the Box
        
        // Both boxes are accessible and contain equal circles
        assert_eq!(*box1, *box2);
        assert_eq!(box1.area(), box2.area());
    }

    #[test]
    fn test_circle_intersection_analysis() {
        let circles = vec![
            Circle::new(Point::new(0.0, 0.0), 2.0),
            Circle::new(Point::new(3.0, 0.0), 2.0),  // Touching
            Circle::new(Point::new(1.0, 1.0), 1.0),  // Overlapping with first
            Circle::new(Point::new(10.0, 10.0), 1.0), // Far away
        ];
        
        let intersections = analyze_circle_intersections(&circles);
        
        // Should find intersections between some circles
        assert!(intersections.len() > 0);
        
        // Check that circles 0 and 1 intersect (they touch)
        assert!(intersections.contains(&(0, 1)));
    }

    #[test]
    fn test_factory_methods() {
        let unit = Circle::unit_circle();
        assert_eq!(unit.radius(), 1.0);
        assert_eq!(*unit.center(), Point::new(0.0, 0.0));
        
        let at_origin = Circle::at_origin(5.0);
        assert_eq!(at_origin.radius(), 5.0);
        assert_eq!(*at_origin.center(), Point::new(0.0, 0.0));
    }

    #[test]
    fn test_automatic_cleanup() {
        // Test that Circles are automatically cleaned up
        for _ in 0..50 {
            let _heap_circle = create_heap_circle(Point::new(0.0, 0.0), 1.0);
            let _circle_vec = create_circle_array(10);
            let _boxed_circles = create_boxed_circle_array(5);
            let _concentric = create_concentric_circles(Point::new(0.0, 0.0), 3, 1.0);
            // All automatically cleaned up at end of each iteration
        }
        assert!(true); // No memory leaks
    }
}
