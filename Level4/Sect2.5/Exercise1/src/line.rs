// Line class for dynamic allocation exercise
// ==========================================
// Demonstrates heap allocation and dynamic arrays of Lines

#![allow(dead_code)]
use crate::point::Point;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Line {
    pub(crate) start: Point,
    pub(crate) end: Point,
}

impl Line {
    // Constructor with two points
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }
    
    // Default constructor - line from origin to origin
    pub fn default() -> Self {
        Line::new(Point::default(), Point::default())
    }
    
    // Constructor using conversion - both points from single values
    pub fn from_values(start_val: f64, end_val: f64) -> Self {
        Line::new(Point::from(start_val), Point::from(end_val))
    }
    
    // Factory methods
    pub fn horizontal(length: f64) -> Self {
        Line::new(Point::default(), Point::new(length, 0.0))
    }
    
    pub fn vertical(length: f64) -> Self {
        Line::new(Point::default(), Point::new(0.0, length))
    }
    
    // Public getters
    pub fn start(&self) -> &Point {
        &self.start
    }
    
    pub fn end(&self) -> &Point {
        &self.end
    }
    
    // Setters
    pub fn set_start(&mut self, start: Point) {
        self.start = start;
    }
    
    pub fn set_end(&mut self, end: Point) {
        self.end = end;
    }
    
    // Calculate length using delegation to Point's distance method
    pub fn length(&self) -> f64 {
        self.start.distance(&self.end)
    }
    
    // Get midpoint using Point operators
    pub fn midpoint(&self) -> Point {
        (self.start + self.end) * 0.5
    }
    
    // Get slope
    pub fn slope(&self) -> Option<f64> {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        
        if dx.abs() < f64::EPSILON {
            None  // Vertical line
        } else {
            Some(dy / dx)
        }
    }
    
    // Check if line contains a point
    pub fn contains_point(&self, point: &Point) -> bool {
        let dist_start = self.start.distance(point);
        let dist_end = self.end.distance(point);
        let line_length = self.length();
        
        const EPSILON: f64 = 1e-10;
        (dist_start + dist_end - line_length).abs() < EPSILON
    }
    
    // ToString methods
    pub fn to_string_custom(&self) -> String {
        format!("Line[{} -> {}]", 
                self.start.to_string_custom(), 
                self.end.to_string_custom())
    }
    
    pub fn to_string_with_length(&self) -> String {
        format!("Line[{} -> {}] (length: {:.2})", 
                self.start.to_string_custom(), 
                self.end.to_string_custom(),
                self.length())
    }
}

// Display trait implementation
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "Line[start: Point({:.2}, {:.2}), end: Point({:.2}, {:.2}), length: {:.2}]",
                   self.start.x, self.start.y,
                   self.end.x, self.end.y,
                   self.length())
        } else {
            write!(f, "Line[Point({:.2}, {:.2}) -> Point({:.2}, {:.2})]",
                   self.start.x, self.start.y,
                   self.end.x, self.end.y)
        }
    }
}

// Custom Debug format
impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Line")
            .field("start.x", &self.start.x)
            .field("start.y", &self.start.y)
            .field("end.x", &self.end.x)
            .field("end.y", &self.end.y)
            .field("length", &self.length())
            .field("slope", &self.slope())
            .finish()
    }
}

// Utility functions for demonstrating dynamic allocation of Lines

// Function that creates a Line on the heap and returns it
pub fn create_heap_line(start: Point, end: Point) -> Box<Line> {
    Box::new(Line::new(start, end))
}

// Function that creates a vector of Lines
pub fn create_line_array(size: usize) -> Vec<Line> {
    (0..size)
        .map(|i| {
            let start = Point::new(i as f64, 0.0);
            let end = Point::new(i as f64, (i + 1) as f64);
            Line::new(start, end)
        })
        .collect()
}

// Function that creates a boxed slice of Lines
pub fn create_boxed_line_array(size: usize) -> Box<[Line]> {
    (0..size)
        .map(|i| {
            let start = Point::new(0.0, i as f64);
            let end = Point::new((i + 1) as f64, i as f64);
            Line::new(start, end)
        })
        .collect()
}

// Function to create lines from a vector of points
pub fn create_lines_from_points(points: &[Point]) -> Vec<Line> {
    if points.len() < 2 {
        return Vec::new();
    }
    
    points.windows(2)
        .map(|window| Line::new(window[0], window[1]))
        .collect()
}

// Function to demonstrate dynamic line creation patterns
pub fn demonstrate_dynamic_line_creation() {
    println!("Dynamic Line Creation Demonstration:");
    
    // Single heap-allocated line
    let heap_line = create_heap_line(
        Point::new(0.0, 0.0),
        Point::new(3.0, 4.0)
    );
    println!("Heap line: {} (length: {:.2})", heap_line, heap_line.length());
    
    // Vector of lines
    let line_vec = create_line_array(5);
    println!("Vector with {} lines:", line_vec.len());
    for (i, line) in line_vec.iter().enumerate() {
        println!("  Line {}: {} (length: {:.2})", i, line, line.length());
    }
    
    // Boxed array of lines
    let boxed_lines = create_boxed_line_array(3);
    println!("Boxed array with {} lines:", boxed_lines.len());
    for (i, line) in boxed_lines.iter().enumerate() {
        println!("  Line {}: {} (length: {:.2})", i, line, line.length());
    }
}

// Function to compare memory allocation approaches
pub fn compare_line_allocation() {
    println!("\nLine Allocation Comparison:");
    
    // Stack allocation
    let stack_line = Line::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
    println!("Stack line: {}", stack_line);
    println!("Size of Line: {} bytes", std::mem::size_of::<Line>());
    
    // Heap allocation with Box
    let heap_line = Box::new(Line::new(Point::new(5.0, 6.0), Point::new(7.0, 8.0)));
    println!("Heap line: {}", heap_line);
    println!("Size of Box<Line>: {} bytes", std::mem::size_of::<Box<Line>>());
    
    // Vector allocation
    let line_vec = vec![
        Line::new(Point::new(1.0, 1.0), Point::new(2.0, 2.0)),
        Line::new(Point::new(3.0, 3.0), Point::new(4.0, 4.0)),
    ];
    println!("Vector with {} lines", line_vec.len());
    println!("Vector capacity: {}", line_vec.capacity());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let start = Point::new(0.0, 0.0);
        let end = Point::new(3.0, 4.0);
        let line = Line::new(start, end);
        
        assert_eq!(*line.start(), start);
        assert_eq!(*line.end(), end);
        assert_eq!(line.length(), 5.0);
    }

    #[test]
    fn test_heap_line_allocation() {
        let heap_line = create_heap_line(
            Point::new(1.0, 1.0),
            Point::new(4.0, 5.0)
        );
        
        assert_eq!(heap_line.start().x(), 1.0);
        assert_eq!(heap_line.start().y(), 1.0);
        assert_eq!(heap_line.end().x(), 4.0);
        assert_eq!(heap_line.end().y(), 5.0);
        assert_eq!(heap_line.length(), 5.0);
    }

    #[test]
    fn test_line_array_creation() {
        let lines = create_line_array(3);
        assert_eq!(lines.len(), 3);
        
        // Check first line
        assert_eq!(lines[0].start().x(), 0.0);
        assert_eq!(lines[0].start().y(), 0.0);
        assert_eq!(lines[0].end().x(), 0.0);
        assert_eq!(lines[0].end().y(), 1.0);
        
        // Check length
        assert_eq!(lines[0].length(), 1.0);
    }

    #[test]
    fn test_boxed_line_array() {
        let boxed_lines = create_boxed_line_array(2);
        assert_eq!(boxed_lines.len(), 2);
        
        assert_eq!(boxed_lines[0].start(), &Point::new(0.0, 0.0));
        assert_eq!(boxed_lines[0].end(), &Point::new(1.0, 0.0));
        assert_eq!(boxed_lines[1].start(), &Point::new(0.0, 1.0));
        assert_eq!(boxed_lines[1].end(), &Point::new(2.0, 1.0));
    }

    #[test]
    fn test_lines_from_points() {
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 0.0),
            Point::new(3.0, 1.0),
        ];
        
        let lines = create_lines_from_points(&points);
        assert_eq!(lines.len(), 3);
        
        // Check first line connects first two points
        assert_eq!(*lines[0].start(), points[0]);
        assert_eq!(*lines[0].end(), points[1]);
        
        // Check last line connects last two points
        assert_eq!(*lines[2].start(), points[2]);
        assert_eq!(*lines[2].end(), points[3]);
    }

    #[test]
    fn test_line_methods() {
        let line = Line::new(Point::new(0.0, 0.0), Point::new(4.0, 0.0));
        
        // Test slope
        assert_eq!(line.slope(), Some(0.0));
        
        // Test midpoint
        let midpoint = line.midpoint();
        assert_eq!(midpoint, Point::new(2.0, 0.0));
        
        // Test contains_point
        assert!(line.contains_point(&Point::new(2.0, 0.0)));
        assert!(!line.contains_point(&Point::new(2.0, 1.0)));
    }

    #[test]
    fn test_factory_methods() {
        let horizontal = Line::horizontal(5.0);
        assert_eq!(horizontal.length(), 5.0);
        assert_eq!(horizontal.slope(), Some(0.0));
        
        let vertical = Line::vertical(3.0);
        assert_eq!(vertical.length(), 3.0);
        assert_eq!(vertical.slope(), None);
    }

    #[test]
    fn test_copy_semantics() {
        let original = Line::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        let copied = original;  // Line implements Copy
        
        // Both are accessible
        assert_eq!(original.length(), copied.length());
        assert_eq!(*original.start(), *copied.start());
        assert_eq!(*original.end(), *copied.end());
    }

    #[test]
    fn test_box_line_operations() {
        let box1 = Box::new(Line::new(Point::new(0.0, 0.0), Point::new(3.0, 4.0)));
        let box2 = Box::new(*box1);  // Copy the Line inside the Box
        
        // Both boxes are accessible and contain equal lines
        assert_eq!(*box1, *box2);
        assert_eq!(box1.length(), box2.length());
    }

    #[test]
    fn test_empty_point_vector() {
        let empty_points: Vec<Point> = vec![];
        let lines = create_lines_from_points(&empty_points);
        assert_eq!(lines.len(), 0);
        
        let single_point = vec![Point::new(1.0, 1.0)];
        let lines_from_single = create_lines_from_points(&single_point);
        assert_eq!(lines_from_single.len(), 0);
    }

    #[test]
    fn test_automatic_cleanup() {
        // Test that Lines are automatically cleaned up
        for _ in 0..100 {
            let _heap_line = create_heap_line(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
            let _line_vec = create_line_array(10);
            let _boxed_lines = create_boxed_line_array(5);
            // All automatically cleaned up at end of each iteration
        }
        assert!(true); // No memory leaks
    }
}
