// Line class in CAD namespace - paul_lopez::cad::Line
// ==================================================
#![allow(dead_code)]
use super::Point; // Use Point from same CAD module (relative import)
use std::fmt;

/// Line class representing a line segment between two points
/// 
/// This represents a line segment in 2D space defined by start and end points
/// Located in the paul_lopez::cad namespace
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    /// Create a new line between two points
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }
    
    /// Create a line from origin to origin (default)
    pub fn default() -> Self {
        Line::new(Point::default(), Point::default())
    }
    
    /// Create a horizontal line from origin with given length
    pub fn horizontal(length: f64) -> Self {
        Line::new(Point::new(0.0, 0.0), Point::new(length, 0.0))
    }
    
    /// Create a vertical line from origin with given length
    pub fn vertical(length: f64) -> Self {
        Line::new(Point::new(0.0, 0.0), Point::new(0.0, length))
    }
    
    /// Get the start point
    pub fn start(&self) -> &Point {
        &self.start
    }
    
    /// Get the end point
    pub fn end(&self) -> &Point {
        &self.end
    }
    
    /// Set the start point
    pub fn set_start(&mut self, start: Point) {
        self.start = start;
    }
    
    /// Set the end point
    pub fn set_end(&mut self, end: Point) {
        self.end = end;
    }
    
    /// Calculate the length of the line
    pub fn length(&self) -> f64 {
        self.start.distance(&self.end)
    }
    
    /// Get the midpoint of the line
    pub fn midpoint(&self) -> Point {
        (self.start + self.end) * 0.5
    }
    
    /// Get the slope of the line (rise/run)
    /// Returns None for vertical lines
    pub fn slope(&self) -> Option<f64> {
        let dx = self.end.x() - self.start.x();
        let dy = self.end.y() - self.start.y();
        
        if dx.abs() < f64::EPSILON {
            None // Vertical line
        } else {
            Some(dy / dx)
        }
    }
    
    /// Check if the line is horizontal
    pub fn is_horizontal(&self) -> bool {
        (self.start.y() - self.end.y()).abs() < f64::EPSILON
    }
    
    /// Check if the line is vertical
    pub fn is_vertical(&self) -> bool {
        (self.start.x() - self.end.x()).abs() < f64::EPSILON
    }
    
    /// Get the angle of the line in radians
    pub fn angle(&self) -> f64 {
        let dx = self.end.x() - self.start.x();
        let dy = self.end.y() - self.start.y();
        dy.atan2(dx)
    }
    
    /// Check if a point lies on this line segment
    pub fn contains_point(&self, point: &Point) -> bool {
        let dist_to_start = self.start.distance(point);
        let dist_to_end = self.end.distance(point);
        let line_length = self.length();
        
        // Point is on line if sum of distances equals line length
        const EPSILON: f64 = 1e-10;
        (dist_to_start + dist_to_end - line_length).abs() < EPSILON
    }
    
    /// Translate the line by given offset
    pub fn translate(&self, dx: f64, dy: f64) -> Line {
        Line::new(
            self.start.translate(dx, dy),
            self.end.translate(dx, dy)
        )
    }
    
    /// Create a parallel line at given distance
    pub fn parallel_line(&self, distance: f64) -> Line {
        let angle = self.angle();
        let perpendicular_angle = angle + std::f64::consts::PI / 2.0;
        
        let offset_x = distance * perpendicular_angle.cos();
        let offset_y = distance * perpendicular_angle.sin();
        
        self.translate(offset_x, offset_y)
    }
}

impl Default for Line {
    fn default() -> Self {
        Line::new(Point::default(), Point::default())
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line[{} -> {}]", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let start = Point::new(1.0, 2.0);
        let end = Point::new(4.0, 6.0);
        let line = Line::new(start, end);
        
        assert_eq!(*line.start(), start);
        assert_eq!(*line.end(), end);
    }

    #[test]
    fn test_line_length() {
        let line = Line::new(Point::new(0.0, 0.0), Point::new(3.0, 4.0));
        assert_eq!(line.length(), 5.0);
    }

    #[test]
    fn test_line_properties() {
        let horizontal = Line::horizontal(5.0);
        assert!(horizontal.is_horizontal());
        assert!(!horizontal.is_vertical());
        assert_eq!(horizontal.slope(), Some(0.0));
        
        let vertical = Line::vertical(3.0);
        assert!(vertical.is_vertical());
        assert!(!vertical.is_horizontal());
        assert_eq!(vertical.slope(), None);
    }

    #[test]
    fn test_midpoint() {
        let line = Line::new(Point::new(0.0, 0.0), Point::new(4.0, 6.0));
        let midpoint = line.midpoint();
        assert_eq!(midpoint, Point::new(2.0, 3.0));
    }

    #[test]
    fn test_angle() {
        let line = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let angle = line.angle();
        assert!((angle - std::f64::consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_contains_point() {
        let line = Line::new(Point::new(0.0, 0.0), Point::new(4.0, 0.0));
        
        assert!(line.contains_point(&Point::new(2.0, 0.0))); // On line
        assert!(!line.contains_point(&Point::new(2.0, 1.0))); // Off line
        assert!(line.contains_point(&Point::new(0.0, 0.0))); // Start point
        assert!(line.contains_point(&Point::new(4.0, 0.0))); // End point
    }

    #[test]
    fn test_translate() {
        let line = Line::new(Point::new(1.0, 1.0), Point::new(2.0, 2.0));
        let translated = line.translate(3.0, 4.0);
        
        assert_eq!(*translated.start(), Point::new(4.0, 5.0));
        assert_eq!(*translated.end(), Point::new(5.0, 6.0));
    }

    #[test]
    fn test_parallel_line() {
        let line = Line::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0));
        let parallel = line.parallel_line(1.0);
        
        // Parallel line should have same length and be parallel
        assert!((parallel.length() - line.length()).abs() < 1e-10);
        // Should be approximately 1 unit away
        let distance_check = parallel.start().distance(line.start());
        assert!((distance_check - 1.0).abs() < 1e-10);
    }
}
