use crate::point::Point;
use std::fmt;

// Line class using composition - contains two Point objects
#[derive(Clone)]
pub struct Line {
    start_point: Point,  // Composition: HAS-A Point
    end_point: Point,    // Composition: HAS-A Point
}

impl Line {
    // Constructor with start and end points
    // C++: Line(const Point& start, const Point& end)
    // Rust: Takes ownership (or use &Point and clone internally)
    pub fn new(start: Point, end: Point) -> Self {
        Line {
            start_point: start,
            end_point: end,
        }
    }
    
    // Getter for start point - const function
    // C++: const Point& GetStart() const
    // Rust: Returns immutable reference
    pub fn start(&self) -> &Point {
        &self.start_point
    }
    
    // Getter for end point - const function
    // C++: const Point& GetEnd() const
    // Rust: Returns immutable reference
    pub fn end(&self) -> &Point {
        &self.end_point
    }
    
    // Setter for start point - non-const
    // C++: void SetStart(const Point& point)
    // Rust: Takes ownership (or could take &Point and clone)
    pub fn set_start(&mut self, point: Point) {
        self.start_point = point;
    }
    
    // Setter for end point - non-const
    // C++: void SetEnd(const Point& point)
    // Rust: Takes ownership (or could take &Point and clone)
    pub fn set_end(&mut self, point: Point) {
        self.end_point = point;
    }
    
    // ToString function - const function
    // C++: string ToString() const
    // Rust: &self makes it const
    pub fn to_string(&self) -> String {
        format!("Line from {} to {}", self.start_point, self.end_point)
    }
    
    // Length function using delegation - const function
    // C++: double Length() const
    // Delegates to Point's distance method
    pub fn length(&self) -> f64 {
        // Delegation: Use Point's distance method
        self.start_point.distance(&self.end_point)
    }
}

// Default constructor - points set to (0, 0)
impl Default for Line {
    fn default() -> Self {
        Line {
            start_point: Point::default(),
            end_point: Point::default(),
        }
    }
}

// Display trait for nice printing
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Optional: Destructor for demonstration
// In Rust, this is usually not needed due to automatic memory management
impl Drop for Line {
    fn drop(&mut self) {
        println!("Dropping line: {}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default() {
        let line = Line::default();
        assert_eq!(line.length(), 0.0);
    }
    
    #[test]
    fn test_constructor() {
        let line = Line::new(
            Point::new(0.0, 0.0),
            Point::new(3.0, 4.0)
        );
        assert_eq!(line.length(), 5.0);
    }
    
    #[test]
    fn test_setters() {
        let mut line = Line::default();
        line.set_start(Point::new(0.0, 0.0));
        line.set_end(Point::new(5.0, 12.0));
        assert_eq!(line.length(), 13.0);
    }
    
    #[test]
    fn test_clone() {
        let line1 = Line::new(
            Point::new(1.0, 1.0),
            Point::new(4.0, 5.0)
        );
        let line2 = line1.clone();
        assert_eq!(line1.length(), line2.length());
    }
}
