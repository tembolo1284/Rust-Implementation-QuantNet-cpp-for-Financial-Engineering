// Line class with Display trait implementation
// =============================================
// In C++: ostream& operator << (ostream& os, const Line& line);
// In Rust: impl std::fmt::Display for Line
//
// The Display trait allows Line objects to be printed with println!("{}", line).

use crate::point::Point;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
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
    
    // Constructor for horizontal line
    pub fn horizontal(length: f64) -> Self {
        Line::new(Point::default(), Point::new(length, 0.0))
    }
    
    // Constructor for vertical line
    pub fn vertical(length: f64) -> Self {
        Line::new(Point::default(), Point::new(0.0, length))
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
    
    // Get slope (rise/run)
    pub fn slope(&self) -> Option<f64> {
        let dx = self.end.x() - self.start.x();
        let dy = self.end.y() - self.start.y();
        
        if dx.abs() < f64::EPSILON {
            None  // Vertical line (undefined slope)
        } else {
            Some(dy / dx)
        }
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
    
    // ToString method (equivalent to C++ ToString())
    pub fn to_string_custom(&self) -> String {
        format!("Line[{} -> {}]", 
                self.start.to_string_custom(), 
                self.end.to_string_custom())
    }
    
    // ToString with length information
    pub fn to_string_with_length(&self) -> String {
        format!("Line[{} -> {}] (length: {:.2})", 
                self.start.to_string_custom(), 
                self.end.to_string_custom(),
                self.length())
    }
}

// Display trait implementation (equivalent to C++ ostream << operator)
// ===================================================================
// In C++:
//   ostream& operator << (ostream& os, const Line& line) {
//       os << line.ToString();  // Using ToString() method
//       return os;
//   }
//
// In Rust, we implement std::fmt::Display:
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the ToString method like in C++ implementation
        if f.alternate() {
            // {:#} format - show more details
            write!(f, "Line[start: {}, end: {}, length: {:.2}]", 
                   self.start, self.end, self.length())
        } else {
            // Default format - use ToString method
            write!(f, "{}", self.to_string_custom())
        }
    }
}

// Additional formatting for different use cases

// LowerExp for scientific notation
impl fmt::LowerExp for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line[{:e} -> {:e}]", self.start, self.end)
    }
}

// Debug format with additional information
impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Line")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("length", &self.length())
            .field("slope", &self.slope())
            .finish()
    }
}

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
    fn test_display_basic() {
        let line = Line::new(
            Point::new(1.0, 2.0),
            Point::new(3.0, 4.0)
        );
        let display = format!("{}", line);
        
        // Should contain both points in the expected format
        assert!(display.contains("Point(1.00, 2.00)"));
        assert!(display.contains("Point(3.00, 4.00)"));
        assert!(display.contains("Line["));
        assert!(display.contains(" -> "));
        assert!(display.contains("]"));
    }

    #[test]
    fn test_display_alternate() {
        let line = Line::new(
            Point::new(0.0, 0.0),
            Point::new(3.0, 4.0)
        );
        let display = format!("{:#}", line);
        
        // Alternate format should include length
        assert!(display.contains("length: 5.00"));
        assert!(display.contains("start:"));
        assert!(display.contains("end:"));
    }

    #[test]
    fn test_to_string_methods() {
        let line = Line::new(
            Point::new(1.0, 2.0),
            Point::new(3.0, 4.0)
        );
        
        let basic = line.to_string_custom();
        let with_length = line.to_string_with_length();
        
        assert!(basic.contains("Line["));
        assert!(with_length.contains("length:"));
    }

    #[test]
    fn test_debug_format() {
        let line = Line::new(
            Point::new(0.0, 0.0),
            Point::new(3.0, 4.0)
        );
        let debug = format!("{:?}", line);
        
        // Debug should show all fields including calculated ones
        assert!(debug.contains("Line"));
        assert!(debug.contains("start"));
        assert!(debug.contains("end"));
        assert!(debug.contains("length"));
        assert!(debug.contains("slope"));
    }

    #[test]
    fn test_slope() {
        // Diagonal line (slope = 1)
        let line1 = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        assert_eq!(line1.slope(), Some(1.0));
        
        // Horizontal line (slope = 0)
        let line2 = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0));
        assert_eq!(line2.slope(), Some(0.0));
        
        // Vertical line (undefined slope)
        let line3 = Line::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0));
        assert_eq!(line3.slope(), None);
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
    fn test_factory_methods() {
        let horizontal = Line::horizontal(5.0);
        assert_eq!(horizontal.length(), 5.0);
        assert_eq!(horizontal.slope(), Some(0.0));
        
        let vertical = Line::vertical(3.0);
        assert_eq!(vertical.length(), 3.0);
        assert_eq!(vertical.slope(), None);
    }

    #[test]
    fn test_scientific_notation() {
        let line = Line::new(
            Point::new(1234.567, 0.0001234),
            Point::new(9876.543, 0.0005678)
        );
        
        let sci_notation = format!("{:e}", line);
        assert!(sci_notation.contains("e"));
    }

    #[test]
    fn test_zero_length_line() {
        let line = Line::new(Point::new(1.0, 1.0), Point::new(1.0, 1.0));
        assert_eq!(line.length(), 0.0);
        
        let display = format!("{}", line);
        assert!(display.contains("Point(1.00, 1.00)"));
    }
}
