use crate::point::Point;
use std::fmt;
use std::f64::consts::PI;

#[derive(Clone)]
pub struct Circle {
    center_point: Point,
    radius: f64,
}

impl Circle {
    // Constructor with center and radius
    pub fn new(center: Point, radius: f64) -> Self {
        Circle {
            center_point: center,
            radius,
        }
    }
    
    // Getter for center - const function
    pub fn center(&self) -> &Point {
        &self.center_point
    }
    
    // Getter for radius - const function
    pub fn radius(&self) -> f64 {
        self.radius
    }
    
    // Setter for center
    pub fn set_center(&mut self, center: Point) {
        self.center_point = center;
    }
    
    // Setter for radius
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    
    // Calculate diameter - const function
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
    
    // Calculate area - const function
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    
    // Calculate circumference - const function
    pub fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }
    
    // ToString function - const function
    pub fn to_string(&self) -> String {
        format!("Circle(center: {}, radius: {:.2})", self.center_point, self.radius)
    }
}

// Default constructor
impl Default for Circle {
    fn default() -> Self {
        Circle {
            center_point: Point::default(),
            radius: 1.0,
        }
    }
}

// Display trait
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Optional destructor for demonstration
impl Drop for Circle {
    fn drop(&mut self) {
        println!("Dropping circle: {}", self);
    }
}
