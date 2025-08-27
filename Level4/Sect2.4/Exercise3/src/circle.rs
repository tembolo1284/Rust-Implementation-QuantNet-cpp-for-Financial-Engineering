// Circle class for Exercise 3: Constructors as conversion operators
// =================================================================
// Demonstrates usage of Point conversions in Circle construction

use crate::point::Point;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Circle {
    center: Point,
    radius: f64,
}

#[allow(dead_code)]
impl Circle {
    // Constructor with center point and radius
    pub fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }
    
    // Default constructor - unit circle at origin
    pub fn default() -> Self {
        Circle::new(Point::default(), 1.0)
    }
    
    // Constructor using conversion - center from single value
    pub fn from_center_value(center_val: f64, radius: f64) -> Self {
        Circle::new(Point::from(center_val), radius)
    }
    
    // Unit circle at origin
    pub fn unit_circle() -> Self {
        Circle::new(Point::default(), 1.0)
    }
    
    // Circle at origin with given radius
    pub fn at_origin(radius: f64) -> Self {
        Circle::new(Point::default(), radius)
    }
    
    // Getters
    pub fn center(&self) -> &Point {
        &self.center
    }
    
    pub fn radius(&self) -> f64 {
        self.radius
    }
    
    // Setters
    pub fn set_center(&mut self, center: Point) {
        self.center = center;
    }
    
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    
    // Mathematical functions
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
    
    // Check if a point is inside the circle
    pub fn contains_point(&self, point: &Point) -> bool {
        self.center.distance(point) <= self.radius
    }
    
    // Check if a point is on the circle boundary
    pub fn point_on_boundary(&self, point: &Point) -> bool {
        const EPSILON: f64 = 1e-10;
        (self.center.distance(point) - self.radius).abs() < EPSILON
    }
    
    // Get point on circle at given angle (in radians)
    pub fn point_at_angle(&self, angle: f64) -> Point {
        let x = self.center.x() + self.radius * angle.cos();
        let y = self.center.y() + self.radius * angle.sin();
        Point::new(x, y)
    }
    
    // Move circle by a vector (using Point operators!)
    pub fn translate(&self, offset: Point) -> Circle {
        Circle::new(self.center + offset, self.radius)
    }
    
    // Scale circle by a factor
    pub fn scale(&self, factor: f64) -> Circle {
        Circle::new(self.center, self.radius * factor)
    }
    
    // ToString method
    pub fn to_string_custom(&self) -> String {
        format!("Circle[center: {}, radius: {:.2}]", 
                self.center.to_string_custom(), 
                self.radius)
    }
}

// Display trait implementation
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "Circle[center: {}, radius: {:.2}, area: {:.2}, circumference: {:.2}]",
                   self.center, self.radius, self.area(), self.circumference())
        } else if let Some(precision) = f.precision() {
            write!(f, "Circle[center: {}, radius: {:.prec$}]",
                   self.center, self.radius, prec = precision)
        } else {
            write!(f, "{}", self.to_string_custom())
        }
    }
}

// Custom Debug format
impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Circle")
            .field("center", &self.center)
            .field("radius", &self.radius)
            .field("diameter", &self.diameter())
            .field("area", &self.area())
            .field("circumference", &self.circumference())
            .finish()
    }
}
