// Line class with proper assignment semantics
// ==========================================
// Demonstrates proper assignment handling in Rust.
// Assignment is automatic and self-assignment is impossible in Rust.

use crate::point::Point;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
}

#[allow(dead_code)]
impl Line {
    // Constructor with two points
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }
    
    // Default constructor - line from origin to origin
    pub fn default() -> Self {
        Line::new(Point::default(), Point::default())
    }
    
    // Getters - return references to avoid unnecessary copies
    pub fn start(&self) -> &Point {
        &self.start
    }
    
    pub fn end(&self) -> &Point {
        &self.end
    }
    
    // Setters - take ownership (Point is Copy, so this is efficient)
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
    
    // Get midpoint
    pub fn midpoint(&self) -> Point {
        (self.start + self.end) * 0.5  // Using Point operators!
    }
    
    // Check if line contains a point (on the line segment)
    pub fn contains_point(&self, point: &Point) -> bool {
        let dist_start = self.start.distance(point);
        let dist_end = self.end.distance(point);
        let line_length = self.length();
        
        // Point is on line if sum of distances equals line length (within epsilon)
        const EPSILON: f64 = 1e-10;
        (dist_start + dist_end - line_length).abs() < EPSILON
    }
}

// Display implementation
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line[{} -> {}]", self.start, self.end)
    }
}

// Assignment operator considerations:
// ==================================
// In C++: Point& operator = (const Point& source);
// - Must handle self-assignment: if (this == &source) return *this;
// - Must return *this for chaining
// - Must perform deep copy of all members
//
// In Rust:
// - Assignment is automatic via Copy trait (for simple types) or Move semantics
// - Self-assignment is impossible due to borrowing rules
// - No explicit implementation needed
// - Chaining happens naturally: let a = b = c; becomes let b = c; let a = b;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let start = Point::new(0.0, 0.0);
        let end = Point::new(3.0, 4.0);
        let line = Line::new(start, end);
        
        assert_eq!(*line.start(), start);
        assert_eq!(*line.end(), end);
    }

    #[test]
    fn test_default_constructor() {
        let line = Line::default();
        assert_eq!(*line.start(), Point::default());
        assert_eq!(*line.end(), Point::default());
    }

    #[test]
    fn test_length() {
        let line = Line::new(
            Point::new(0.0, 0.0),
            Point::new(3.0, 4.0)
        );
        assert_eq!(line.length(), 5.0);
    }

    #[test]
    fn test_midpoint() {
        let line = Line::new(
            Point::new(0.0, 0.0),
            Point::new(4.0, 6.0)
        );
        let mid = line.midpoint();
        assert_eq!(mid, Point::new(2.0, 3.0));
    }

    #[test]
    fn test_assignment() {
        let line1 = Line::new(
            Point::new(1.0, 2.0),
            Point::new(3.0, 4.0)
        );
        
        let line2 = line1; // Assignment (copy since Line implements Copy)
        assert_eq!(line1, line2);
        
        // In C++, we'd worry about self-assignment: line1 = line1;
        // In Rust, this is impossible due to borrowing rules
    }

    #[test]
    fn test_contains_point() {
        let line = Line::new(
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0)
        );
        
        // Point on the line
        assert!(line.contains_point(&Point::new(2.0, 0.0)));
        
        // Point not on the line
        assert!(!line.contains_point(&Point::new(2.0, 1.0)));
        
        // Endpoints
        assert!(line.contains_point(&Point::new(0.0, 0.0)));
        assert!(line.contains_point(&Point::new(4.0, 0.0)));
    }

    #[test]
    fn test_setters() {
        let mut line = Line::default();
        
        let new_start = Point::new(1.0, 1.0);
        let new_end = Point::new(2.0, 2.0);
        
        line.set_start(new_start);
        line.set_end(new_end);
        
        assert_eq!(*line.start(), new_start);
        assert_eq!(*line.end(), new_end);
    }
}
