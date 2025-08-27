// Circle class for array of pointers exercise  
// ============================================
// Demonstrates usage with Point arrays

#![allow(dead_code)]
use crate::point::Point;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Circle {
    pub(crate) center: Point,
    pub(crate) radius: f64,
}

impl Circle {
    // Constructor with center point and radius
    pub fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }
    
    // Default constructor - unit circle at origin
    pub fn default() -> Self {
        Circle::new(Point::default(), 1.0)
    }
    
    // Getters
    pub fn center(&self) -> &Point {
        &self.center
    }
    
    pub fn radius(&self) -> f64 {
        self.radius
    }
    
    // Mathematical functions
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// Display trait implementation
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle[center: Point({:.2}, {:.2}), radius: {:.2}]",
               self.center.x, self.center.y, self.radius)
    }
}
