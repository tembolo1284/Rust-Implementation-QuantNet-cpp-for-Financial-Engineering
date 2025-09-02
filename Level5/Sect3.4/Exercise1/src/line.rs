use crate::point::Point;
use std::fmt;

// Non-optimized Line (simulates C++ without colon syntax)
// This version creates default Points first, then assigns values
#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

#[allow(dead_code)]
impl Line {
    // Non-optimized constructor - simulates C++ constructor body assignment
    pub fn new(start: Point, end: Point) -> Self {
        println!("  Line::new() - Non-optimized constructor starting");
        
        // Simulate C++ constructor body assignment (less efficient)
        // In C++, this would first call default constructors, then assign
        let mut line = Line {
            start: Point::default(),  // Default constructor called
            end: Point::default(),    // Default constructor called
        };
        
        // Simulate assignment operations
        println!("  Line::new() - Assigning start point");
        line.start = start;  // Assignment (move in Rust)
        
        println!("  Line::new() - Assigning end point");
        line.end = end;      // Assignment (move in Rust)
        
        println!("  Line::new() - Non-optimized constructor completed");
        line
    }

    // Default constructor
    pub fn default() -> Self {
        println!("  Line::default() - Creating line with default points");
        Self::new(Point::default(), Point::default())
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

    // Length calculation using delegation
    pub fn length(&self) -> f64 {
        self.start.distance(&self.end)
    }

    // ToString function
    pub fn to_string(&self) -> String {
        format!("Line from {} to {}", self.start.to_string(), self.end.to_string())
    }
}

// Optimized Line (simulates C++ with colon syntax)
// This version directly initializes fields without intermediate default construction
#[derive(Debug)]
pub struct LineOptimized {
    start: Point,
    end: Point,
}

#[allow(dead_code)]
impl LineOptimized {
    // Optimized constructor - simulates C++ member initializer list (colon syntax)
    pub fn new(start: Point, end: Point) -> Self {
        println!("  LineOptimized::new() - Optimized constructor (direct initialization)");
        // Direct field initialization - no intermediate default construction
        Self { start, end }
    }

    // Optimized default constructor
    pub fn default() -> Self {
        println!("  LineOptimized::default() - Creating line with optimized construction");
        Self {
            start: Point::default(),
            end: Point::default(),
        }
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

    // Length calculation using delegation
    pub fn length(&self) -> f64 {
        self.start.distance(&self.end)
    }

    // ToString function
    pub fn to_string(&self) -> String {
        format!("LineOptimized from {} to {}", self.start.to_string(), self.end.to_string())
    }
}

// Implement Clone for Line (equivalent to C++ copy constructor)
impl Clone for Line {
    fn clone(&self) -> Self {
        println!("  Line::clone() - Copy constructor called");
        Self {
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}

// Implement Clone for LineOptimized
impl Clone for LineOptimized {
    fn clone(&self) -> Self {
        println!("  LineOptimized::clone() - Copy constructor called");
        Self {
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}

// Implement Drop for Line (equivalent to C++ destructor)
impl Drop for Line {
    fn drop(&mut self) {
        println!("  Line::drop() - Destructor called");
    }
}

// Implement Drop for LineOptimized
impl Drop for LineOptimized {
    fn drop(&mut self) {
        println!("  LineOptimized::drop() - Destructor called");
    }
}

// Implement Display trait
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line from {} to {}", self.start, self.end)
    }
}

impl fmt::Display for LineOptimized {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LineOptimized from {} to {}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let start = Point::new(0.0, 0.0);
        let end = Point::new(3.0, 4.0);
        let line = Line::new(start, end);
        assert_eq!(line.length(), 5.0);
    }

    #[test]
    fn test_line_optimized_creation() {
        let start = Point::new(0.0, 0.0);
        let end = Point::new(3.0, 4.0);
        let line = LineOptimized::new(start, end);
        assert_eq!(line.length(), 5.0);
    }
}
