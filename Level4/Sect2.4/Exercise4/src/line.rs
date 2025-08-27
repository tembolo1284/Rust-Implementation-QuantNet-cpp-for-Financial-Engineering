// Line class with module-level visibility (Rust's "friend" equivalent)
// ====================================================================
// In C++: friend ostream& operator << (ostream& os, const Line& line);
// In Rust: pub(crate) fields + same-module Display implementation

#[allow(unused_imports)]
use crate::point::{Point, point_debug_info};
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Line {
    // pub(crate) = visible within this crate/module, like C++ friend access
    pub(crate) start: Point,
    pub(crate) end: Point,
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
    
    // Public getters (still needed for external crate access)
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
        // Direct field access to Point's coordinates
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
    
    // ToString method (may not be needed by Display, but kept for compatibility)
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

// Display trait - equivalent to C++ friend ostream& operator <<
// ============================================================
// This implementation can access Line's start/end fields directly
// AND can access Point's x/y fields directly (both are pub(crate))
// This is Rust's equivalent to C++ friend function access.

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // DIRECT FIELD ACCESS - like C++ friend function!
            // Access Line fields (start, end) and Point fields (x, y) directly
            write!(f, "Line[start: Point({:.2}, {:.2}), end: Point({:.2}, {:.2}), length: {:.2}]",
                   self.start.x, self.start.y,      // Direct access to Point fields
                   self.end.x, self.end.y,          // Direct access to Point fields
                   self.length())                   // Method call for length
        } else {
            // Direct field access instead of calling to_string_custom()
            write!(f, "Line[Point({:.2}, {:.2}) -> Point({:.2}, {:.2})]",
                   self.start.x, self.start.y,
                   self.end.x, self.end.y)
        }
    }
}

// Additional formatting with direct field access
impl fmt::LowerExp for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Direct access to nested fields
        write!(f, "Line[Point({:e}, {:e}) -> Point({:e}, {:e})]",
               self.start.x, self.start.y,
               self.end.x, self.end.y)
    }
}

// Custom Debug format with extensive direct field access
impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Line")
            .field("start.x", &self.start.x)        // Direct Point field access
            .field("start.y", &self.start.y)        // Direct Point field access
            .field("end.x", &self.end.x)            // Direct Point field access
            .field("end.y", &self.end.y)            // Direct Point field access
            .field("length", &self.length())
            .field("slope", &self.slope())
            .finish()
    }
}

// Module-level helper function (can access pub(crate) fields)
// This demonstrates the "friend-like" access within the module
#[allow(dead_code)]
pub(crate) fn line_debug_info(line: &Line) -> String {
    // Direct access to all fields, like C++ friend function
    format!("Line Debug: start=({:.3},{:.3}), end=({:.3},{:.3}), length={:.3}, slope={:?}",
            line.start.x, line.start.y,
            line.end.x, line.end.y,
            line.length(),
            line.slope())
}

// Advanced line analysis function using direct field access
#[allow(dead_code)]
pub(crate) fn analyze_line(line: &Line) -> String {
    let dx = line.end.x - line.start.x;
    let dy = line.end.y - line.start.y;
    let length = line.length();
    
    let direction = if dx.abs() < f64::EPSILON {
        "Vertical"
    } else if dy.abs() < f64::EPSILON {
        "Horizontal"  
    } else if (dy / dx).abs() == 1.0 {
        "Diagonal (45°)"
    } else {
        "General"
    };
    
    format!("Line Analysis: {} line, length={:.2}, dx={:.2}, dy={:.2}",
            direction, length, dx, dy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_field_access_in_module() {
        let line = Line::new(
            Point::new(1.0, 2.0),
            Point::new(3.0, 4.0)
        );
        
        // We can access Line fields and Point fields directly
        assert_eq!(line.start.x, 1.0);
        assert_eq!(line.start.y, 2.0);
        assert_eq!(line.end.x, 3.0);
        assert_eq!(line.end.y, 4.0);
        
        // Module helper functions can access all fields
        let debug_info = line_debug_info(&line);
        assert!(debug_info.contains("start=(1.000,2.000)"));
        assert!(debug_info.contains("end=(3.000,4.000)"));
    }

    #[test]
    fn test_display_uses_direct_field_access() {
        let line = Line::new(
            Point::new(5.123, 6.456),
            Point::new(7.789, 8.012)
        );
        
        // Display uses direct field access (no getter calls)
        let display_string = format!("{}", line);
        assert!(display_string.contains("5.12"));
        assert!(display_string.contains("6.46"));
        assert!(display_string.contains("7.79"));
        assert!(display_string.contains("8.01"));
        
        // Alternate format also uses direct access
        let alt_format = format!("{:#}", line);
        assert!(alt_format.contains("start: Point(5.12, 6.46)"));
        assert!(alt_format.contains("end: Point(7.79, 8.01)"));
        assert!(alt_format.contains("length:"));
    }

    #[test]
    fn test_slope_calculation_with_direct_access() {
        // Horizontal line
        let horizontal = Line::new(Point::new(0.0, 5.0), Point::new(10.0, 5.0));
        assert_eq!(horizontal.slope(), Some(0.0));
        
        // Vertical line  
        let vertical = Line::new(Point::new(5.0, 0.0), Point::new(5.0, 10.0));
        assert_eq!(vertical.slope(), None);
        
        // Diagonal line (slope = 1)
        let diagonal = Line::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
        assert_eq!(diagonal.slope(), Some(1.0));
        
        // The slope method uses direct field access to Point coordinates
    }

    #[test]
    fn test_debug_format_shows_all_fields() {
        let line = Line::new(Point::new(1.5, 2.5), Point::new(3.5, 4.5));
        let debug_string = format!("{:?}", line);
        
        // Debug format shows direct field access
        assert!(debug_string.contains("start.x: 1.5"));
        assert!(debug_string.contains("start.y: 2.5"));
        assert!(debug_string.contains("end.x: 3.5"));
        assert!(debug_string.contains("end.y: 4.5"));
        assert!(debug_string.contains("length:"));
        assert!(debug_string.contains("slope:"));
    }

    #[test]
    fn test_module_helper_functions() {
        let line = Line::new(Point::new(0.0, 0.0), Point::new(3.0, 4.0));
        
        let debug_info = line_debug_info(&line);
        assert!(debug_info.contains("start=(0.000,0.000)"));
        assert!(debug_info.contains("end=(3.000,4.000)"));
        assert!(debug_info.contains("length=5.000"));
        
        let analysis = analyze_line(&line);
        assert!(analysis.contains("General line"));
        assert!(analysis.contains("length=5.00"));
        assert!(analysis.contains("dx=3.00"));
        assert!(analysis.contains("dy=4.00"));
    }

    #[test]
    fn test_field_modification() {
        let mut line = Line::new(Point::new(1.0, 1.0), Point::new(2.0, 2.0));
        
        // Direct field modification within module
        line.start.x = 10.0;
        line.start.y = 20.0;
        line.end.x = 30.0;
        line.end.y = 40.0;
        
        assert_eq!(line.start.x, 10.0);
        assert_eq!(line.start.y, 20.0);
        assert_eq!(line.end.x, 30.0);
        assert_eq!(line.end.y, 40.0);
    }

    #[test]
    fn test_public_getters_still_work() {
        let line = Line::new(Point::new(7.0, 8.0), Point::new(9.0, 10.0));
        
        // Public getters still exist for external access
        assert_eq!(line.start().x(), 7.0);
        assert_eq!(line.start().y(), 8.0);
        assert_eq!(line.end().x(), 9.0);
        assert_eq!(line.end().y(), 10.0);
        
        // But we can also access directly within module
        assert_eq!(line.start.x, 7.0);
        assert_eq!(line.start.y, 8.0);
        assert_eq!(line.end.x, 9.0);
        assert_eq!(line.end.y, 10.0);
    }

    #[test]
    fn test_factory_methods_with_direct_access() {
        let horizontal = Line::horizontal(5.0);
        let vertical = Line::vertical(3.0);
        
        // Verify using direct field access
        assert_eq!(horizontal.start.x, 0.0);
        assert_eq!(horizontal.start.y, 0.0);
        assert_eq!(horizontal.end.x, 5.0);
        assert_eq!(horizontal.end.y, 0.0);
        
        assert_eq!(vertical.start.x, 0.0);
        assert_eq!(vertical.start.y, 0.0);
        assert_eq!(vertical.end.x, 0.0);
        assert_eq!(vertical.end.y, 3.0);
    }

    #[test]
    fn test_scientific_notation_format() {
        let line = Line::new(
            Point::new(1234.567, 0.0001234),
            Point::new(9876.543, 0.0005678)
        );
        
        let sci_format = format!("{:e}", line);
        assert!(sci_format.contains("e"));
        // Scientific notation format uses direct field access
    }
}
