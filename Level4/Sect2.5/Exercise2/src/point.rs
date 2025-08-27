// Point class for array of pointers exercise
// ===========================================
// Demonstrates heap allocation with Box<T> in arrays
// Focus on individual heap allocations per Point object

#![allow(dead_code)]
use std::fmt;
use std::ops::{Neg, Mul, Add, MulAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point {
    // Constructor
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    // Default constructor - point at origin
    pub fn default() -> Self {
        Point::new(0.0, 0.0)
    }
    
    // Single-value constructor
    pub fn from_single_value(value: f64) -> Self {
        Point::new(value, value)
    }
    
    // Public getters
    pub fn x(&self) -> f64 {
        self.x
    }
    
    pub fn y(&self) -> f64 {
        self.y
    }
    
    // Setters
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    
    // Distance calculations
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    pub fn distance_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    // ToString methods
    pub fn to_string_custom(&self) -> String {
        format!("Point({:.2}, {:.2})", self.x, self.y)
    }
}

// Display trait implementation
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:.2}, {:.2})", self.x, self.y)
    }
}

// Operator implementations

impl Neg for Point {
    type Output = Point;
    
    fn neg(self) -> Self::Output {
        Point { x: -self.x, y: -self.y }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    
    fn mul(self, factor: f64) -> Self::Output {
        Point { x: self.x * factor, y: self.y * factor }
    }
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Self::Output {
        Point { 
            x: self.x + other.x, 
            y: self.y + other.y 
        }
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }
}

// Allow f64 * Point
impl Mul<Point> for f64 {
    type Output = Point;
    
    fn mul(self, point: Point) -> Self::Output {
        point * self
    }
}

// Conversion traits
impl From<f64> for Point {
    fn from(value: f64) -> Self {
        Point { x: value, y: value }
    }
}

impl From<i32> for Point {
    fn from(value: i32) -> Self {
        let value_f64 = value as f64;
        Point { x: value_f64, y: value_f64 }
    }
}

// Cross-type comparisons
impl PartialEq<f64> for Point {
    fn eq(&self, other: &f64) -> bool {
        self.x == *other && self.y == *other
    }
}

impl PartialEq<Point> for f64 {
    fn eq(&self, other: &Point) -> bool {
        other == self
    }
}
