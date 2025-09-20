use crate::point::Point;
use crate::shape::{Shape, ShapeData};
use std::fmt;

const PI: f64 = std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Circle {
    // Composition: Circle HAS-A ShapeData (equivalent to C++ inheritance)
    shape_data: ShapeData,
    center: Point,
    radius: f64,
}

impl Circle {
    // Constructor with center and radius (calls Shape constructor equivalent)
    pub fn new(center: Point, radius: f64) -> Self {
        println!("  Circle::new() - Constructor called with radius {}", radius);
        Self {
            shape_data: ShapeData::new(),  // Equivalent to calling base class constructor
            center,
            radius,
        }
    }

    // Constructor with specific ID
    pub fn new_with_id(center: Point, radius: f64, id: i32) -> Self {
        println!("  Circle::new_with_id() - Constructor called with radius {} and ID {}", radius, id);
        Self {
            shape_data: ShapeData::with_id(id),
            center,
            radius,
        }
    }

    // Default constructor
    pub fn default() -> Self {
        println!("  Circle::default() - Default constructor called");
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

    // Circle-specific string representation
    pub fn circle_to_string(&self) -> String {
        format!("Circle(center: {}, radius: {:.1})", self.center.point_to_string(), self.radius)
    }
}

// Implement the Shape trait for Circle (equivalent to C++ inheritance)
impl Shape for Circle {
    fn shape_data(&self) -> &ShapeData {
        &self.shape_data
    }

    fn shape_data_mut(&mut self) -> &mut ShapeData {
        &mut self.shape_data
    }

    // Override the to_string method (equivalent to C++ virtual function override)
    fn to_string(&self) -> String {
        format!("Circle(center: {}, radius: {:.1}) [{}]", 
                self.center.point_to_string(), 
                self.radius, 
                self.shape_data.to_string())
    }

    fn shape_type(&self) -> &'static str {
        "Circle"
    }
}

// Implement PartialEq for Circle
impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && (self.radius - other.radius).abs() < f64::EPSILON
        // Note: We don't compare IDs for geometric equality
    }
}

// Implement Drop trait (equivalent to C++ destructor)
impl Drop for Circle {
    fn drop(&mut self) {
        println!("  Circle::drop() - Destructor called for radius {:.1}", self.radius);
    }
}

// Implement Display trait
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Implement Default trait
impl Default for Circle {
    fn default() -> Self {
        Self::default()
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
        assert!(circle.id() >= 1000 && circle.id() <= 9999);
    }

    #[test]
    fn test_circle_shape_trait() {
        let circle = Circle::new(Point::new(1.0, 2.0), 3.0);
        assert_eq!(circle.shape_type(), "Circle");
        assert!(circle.to_string().contains("Circle(center:"));
        assert!(circle.to_string().contains("radius: 3.0"));
        assert!(circle.to_string().contains("ID:"));
    }

    #[test]
    fn test_circle_calculations() {
        let circle = Circle::new(Point::new(0.0, 0.0), 2.0);
        assert_eq!(circle.diameter(), 4.0);
        assert!((circle.area() - (PI * 4.0)).abs() < 1e-10);
        assert!((circle.circumference() - (2.0 * PI * 2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_circle_clone() {
        let circle1 = Circle::new(Point::new(1.0, 2.0), 5.0);
        let circle2 = circle1.clone();
        assert_eq!(circle1.radius(), circle2.radius());
        assert_eq!(circle1.center(), circle2.center());
        assert_eq!(circle1.id(), circle2.id()); // ID should be copied in clone
    }

    #[test]
    fn test_circle_as_shape() {
        let circle = Circle::new(Point::new(0.0, 0.0), 1.0);
        let shape: &dyn Shape = &circle;
        assert_eq!(shape.shape_type(), "Circle");
        assert_eq!(shape.id(), circle.id());
    }

    #[test]
    fn test_mixed_shapes_polymorphism() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Point::new(1.0, 2.0)),
            Box::new(Circle::new(Point::new(0.0, 0.0), 5.0)),
        ];

        let types: Vec<&str> = shapes.iter().map(|s| s.shape_type()).collect();
        assert_eq!(types, vec!["Point", "Circle"]);
    }
}
