use crate::point::Point;
use crate::shape::{Shape, ShapeData};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Line {
    // Composition: Line HAS-A ShapeData (equivalent to C++ inheritance)
    shape_data: ShapeData,
    start: Point,
    end: Point,
}

impl Line {
    // Constructor with two points (calls Shape constructor equivalent)
    pub fn new(start: Point, end: Point) -> Self {
        println!("  Line::new() - Constructor called");
        Self {
            shape_data: ShapeData::new(),  // Equivalent to calling base class constructor
            start,
            end,
        }
    }

    // Constructor with specific ID
    pub fn new_with_id(start: Point, end: Point, id: i32) -> Self {
        println!("  Line::new_with_id() - Constructor called with ID {}", id);
        Self {
            shape_data: ShapeData::with_id(id),
            start,
            end,
        }
    }

    // Default constructor
    pub fn default() -> Self {
        println!("  Line::default() - Default constructor called");
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

    // Line-specific string representation
    pub fn line_to_string(&self) -> String {
        format!("Line from {} to {}", self.start.point_to_string(), self.end.point_to_string())
    }
}

// Implement the Shape trait for Line (equivalent to C++ inheritance)
impl Shape for Line {
    fn shape_data(&self) -> &ShapeData {
        &self.shape_data
    }

    fn shape_data_mut(&mut self) -> &mut ShapeData {
        &mut self.shape_data
    }

    // Override the to_string method (equivalent to C++ virtual function override)
    fn to_string(&self) -> String {
        format!("Line from {} to {} [{}]", 
                self.start.point_to_string(), 
                self.end.point_to_string(), 
                self.shape_data.to_string())
    }

    fn shape_type(&self) -> &'static str {
        "Line"
    }
}

// Implement PartialEq for Line
impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
        // Note: We don't compare IDs for geometric equality
    }
}

// Implement Drop trait (equivalent to C++ destructor)
impl Drop for Line {
    fn drop(&mut self) {
        println!("  Line::drop() - Destructor called");
    }
}

// Implement Display trait
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Implement Default trait
impl Default for Line {
    fn default() -> Self {
        Self::default()
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
        assert!(line.id() >= 1000 && line.id() <= 9999);
    }

    #[test]
    fn test_line_shape_trait() {
        let line = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        assert_eq!(line.shape_type(), "Line");
        assert!(line.to_string().contains("Line from"));
        assert!(line.to_string().contains("ID:"));
    }

    #[test]
    fn test_line_clone() {
        let line1 = Line::new(Point::new(1.0, 2.0), Point::new(3.0, 4.0));
        let line2 = line1.clone();
        assert_eq!(line1.length(), line2.length());
        assert_eq!(line1.id(), line2.id()); // ID should be copied in clone
    }

    #[test]
    fn test_line_as_shape() {
        let line = Line::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0));
        let shape: &dyn Shape = &line;
        assert_eq!(shape.shape_type(), "Line");
        assert_eq!(shape.id(), line.id());
    }

    #[test]
    fn test_line_polymorphism() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0))),
            Box::new(Point::new(2.0, 3.0)),
        ];

        for shape in shapes.iter() {
            assert!(shape.id() >= 1000 && shape.id() <= 9999);
        }
    }
}
