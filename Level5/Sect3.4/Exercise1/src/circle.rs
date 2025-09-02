use crate::point::Point;
use std::fmt;

const PI: f64 = std::f64::consts::PI;

// Non-optimized Circle (simulates C++ without colon syntax)
#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: f64,
}

#[allow(dead_code)]
impl Circle {
    // Non-optimized constructor
    pub fn new(center: Point, radius: f64) -> Self {
        println!("  Circle::new() - Non-optimized constructor starting");
        
        // Simulate non-optimized construction (default + assignment)
        let mut circle = Circle {
            center: Point::default(),  // Default constructor called
            radius: 0.0,
        };
        
        println!("  Circle::new() - Assigning center point");
        circle.center = center;    // Assignment
        
        println!("  Circle::new() - Assigning radius");
        circle.radius = radius;    // Assignment
        
        println!("  Circle::new() - Non-optimized constructor completed");
        circle
    }

    // Default constructor
    pub fn default() -> Self {
        println!("  Circle::default() - Creating circle with default values");
        Self::new(Point::default(), 1.0)
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

    // Diameter
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    // Area
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    // Circumference
    pub fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }

    // ToString function
    pub fn to_string(&self) -> String {
        format!("Circle(center: {}, radius: {:.1})", self.center.to_string(), self.radius)
    }
}

// Optimized Circle (simulates C++ with colon syntax)
#[derive(Debug)]
pub struct CircleOptimized {
    center: Point,
    radius: f64,
}

#[allow(dead_code)]
impl CircleOptimized {
    // Optimized constructor - direct field initialization
    pub fn new(center: Point, radius: f64) -> Self {
        println!("  CircleOptimized::new() - Optimized constructor (direct initialization)");
        Self { center, radius }
    }

    // Optimized default constructor
    pub fn default() -> Self {
        println!("  CircleOptimized::default() - Creating circle with optimized construction");
        Self {
            center: Point::default(),
            radius: 1.0,
        }
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

    // Diameter
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    // Area
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    // Circumference
    pub fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }

    // ToString function
    pub fn to_string(&self) -> String {
        format!("CircleOptimized(center: {}, radius: {:.1})", self.center.to_string(), self.radius)
    }
}

// Implement Clone for Circle
impl Clone for Circle {
    fn clone(&self) -> Self {
        println!("  Circle::clone() - Copy constructor called");
        Self {
            center: self.center.clone(),
            radius: self.radius,
        }
    }
}

// Implement Clone for CircleOptimized
impl Clone for CircleOptimized {
    fn clone(&self) -> Self {
        println!("  CircleOptimized::clone() - Copy constructor called");
        Self {
            center: self.center.clone(),
            radius: self.radius,
        }
    }
}

// Implement Drop for Circle
impl Drop for Circle {
    fn drop(&mut self) {
        println!("  Circle::drop() - Destructor called for radius {:.1}", self.radius);
    }
}

// Implement Drop for CircleOptimized
impl Drop for CircleOptimized {
    fn drop(&mut self) {
        println!("  CircleOptimized::drop() - Destructor called for radius {:.1}", self.radius);
    }
}

// Implement Display trait
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle(center: {}, radius: {:.1})", self.center, self.radius)
    }
}

impl fmt::Display for CircleOptimized {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CircleOptimized(center: {}, radius: {:.1})", self.center, self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_creation() {
        let center = Point::new(0.0, 0.0);
        let circle = Circle::new(center, 5.0);
        assert_eq!(circle.radius(), 5.0);
        assert_eq!(circle.diameter(), 10.0);
        assert!((circle.area() - (PI * 25.0)).abs() < 1e-10);
    }

    #[test]
    fn test_circle_optimized_creation() {
        let center = Point::new(0.0, 0.0);
        let circle = CircleOptimized::new(center, 5.0);
        assert_eq!(circle.radius(), 5.0);
        assert_eq!(circle.diameter(), 10.0);
        assert!((circle.area() - (PI * 25.0)).abs() < 1e-10);
    }
}
