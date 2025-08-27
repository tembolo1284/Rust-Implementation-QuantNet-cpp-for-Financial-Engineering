// Line class for Exercise 3: Constructors as conversion operators
// ===============================================================
// Demonstrates usage of Point conversions in Line construction

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
    
    // Constructor using conversion - both points from single values
    pub fn from_values(start_val: f64, end_val: f64) -> Self {
        Line::new(Point::from(start_val), Point::from(end_val))
    }
    
    // Constructor for horizontal line
    pub fn horizontal(length: f64) -> Self {
        Line::new(Point::default(), Point::new(length, 0.0))
    }
    
    // Constructor for vertical line
    pub fn vertical(length: f64) -> Self {
        Line::new(Point::default(), Point::new(0.0, length))
    }
    
    // Getters
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
        let dx = self.end.x() - self.start.x();
        let dy = self.end.y() - self.start.y();
        
        if dx.abs() < f64::EPSILON {
            None  // Vertical line
        } else {
            Some(dy / dx)
        }
    }
    
    // ToString method
    pub fn to_string_custom(&self) -> String {
        format!("Line[{} -> {}]", 
                self.start.to_string_custom(), 
                self.end.to_string_custom())
    }
}

// Display trait implementation
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "Line[start: {}, end: {}, length: {:.2}]", 
                   self.start, self.end, self.length())
        } else {
            write!(f, "{}", self.to_string_custom())
        }
    }
}

// Custom Debug format
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
