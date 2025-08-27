// Line class for array of pointers exercise
// ==========================================
// Demonstrates usage with Point arrays

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
    
    // Default constructor
    pub fn default() -> Self {
        Line::new(Point::default(), Point::default())
    }
    
    // Getters
    pub fn start(&self) -> &Point {
        &self.start
    }
    
    pub fn end(&self) -> &Point {
        &self.end
    }
    
    // Calculate length
    pub fn length(&self) -> f64 {
        self.start.distance(&self.end)
    }
    
    // Get midpoint
    pub fn midpoint(&self) -> Point {
        (self.start + self.end) * 0.5
    }
}

// Display trait implementation
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line[Point({:.2}, {:.2}) -> Point({:.2}, {:.2})]",
               self.start.x, self.start.y,
               self.end.x, self.end.y)
    }
}
