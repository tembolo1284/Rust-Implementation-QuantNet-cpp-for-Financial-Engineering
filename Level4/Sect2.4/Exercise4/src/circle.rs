// Circle class with module-level visibility (Rust's "friend" equivalent)  
// ======================================================================
// In C++: friend ostream& operator << (ostream& os, const Circle& circle);
// In Rust: pub(crate) fields + same-module Display implementation

#[allow(unused_imports)]
use crate::point::{Point, point_debug_info};
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Circle {
    // pub(crate) = visible within this crate/module, like C++ friend access
    pub(crate) center: Point,
    pub(crate) radius: f64,
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
    
    // Factory methods
    pub fn unit_circle() -> Self {
        Circle::new(Point::default(), 1.0)
    }
    
    pub fn at_origin(radius: f64) -> Self {
        Circle::new(Point::default(), radius)
    }
    
    // Public getters (still needed for external crate access)
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
        // Direct field access to center coordinates
        let x = self.center.x + self.radius * angle.cos();
        let y = self.center.y + self.radius * angle.sin();
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
    
    // Get bounding box using direct field access
    pub fn bounding_box(&self) -> (Point, Point) {
        // Direct access to center coordinates
        let min_x = self.center.x - self.radius;
        let min_y = self.center.y - self.radius;
        let max_x = self.center.x + self.radius;
        let max_y = self.center.y + self.radius;
        
        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }
    
    // ToString methods (may not be needed by Display, but kept for compatibility)
    pub fn to_string_custom(&self) -> String {
        format!("Circle[center: {}, radius: {:.2}]", 
                self.center.to_string_custom(), 
                self.radius)
    }
    
    pub fn to_string_with_properties(&self) -> String {
        format!(
            "Circle[center: {}, radius: {:.2}, area: {:.2}, circumference: {:.2}]", 
            self.center.to_string_custom(), 
            self.radius,
            self.area(),
            self.circumference()
        )
    }
}

// Display trait - equivalent to C++ friend ostream& operator <<
// ============================================================
// This implementation can access Circle's center/radius fields directly
// AND can access Point's x/y fields directly (both are pub(crate))
// This is Rust's equivalent to C++ friend function access.

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // DIRECT FIELD ACCESS - like C++ friend function!
            // Access Circle fields (center, radius) and Point fields (x, y) directly
            write!(f, "Circle[center: Point({:.2}, {:.2}), radius: {:.2}, area: {:.2}, circumference: {:.2}]",
                   self.center.x, self.center.y,        // Direct access to Point fields
                   self.radius,                         // Direct access to Circle field
                   self.area(),                         // Method call
                   self.circumference())                // Method call
        } else if let Some(precision) = f.precision() {
            // Custom precision using direct field access
            write!(f, "Circle[center: Point({:.prec$}, {:.prec$}), radius: {:.prec$}]",
                   self.center.x, self.center.y, self.radius, prec = precision)
        } else {
            // Direct field access instead of calling to_string_custom()
            write!(f, "Circle[center: Point({:.2}, {:.2}), radius: {:.2}]",
                   self.center.x, self.center.y, self.radius)
        }
    }
}

// Additional formatting with direct field access
impl fmt::LowerExp for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Direct access to nested fields
        write!(f, "Circle[center: Point({:e}, {:e}), radius: {:e}]",
               self.center.x, self.center.y, self.radius)
    }
}

// Custom Debug format with extensive direct field access
impl fmt::Debug for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Circle")
            .field("center.x", &self.center.x)       // Direct Point field access
            .field("center.y", &self.center.y)       // Direct Point field access
            .field("radius", &self.radius)           // Direct Circle field access
            .field("diameter", &self.diameter())
            .field("area", &self.area())
            .field("circumference", &self.circumference())
            .finish()
    }
}

// Module-level helper function (can access pub(crate) fields)
// This demonstrates the "friend-like" access within the module
#[allow(dead_code)]
pub(crate) fn circle_debug_info(circle: &Circle) -> String {
    // Direct access to all fields, like C++ friend function
    format!("Circle Debug: center=({:.3},{:.3}), radius={:.3}, area={:.3}, circumference={:.3}",
            circle.center.x, circle.center.y,
            circle.radius,
            circle.area(),
            circle.circumference())
}

// Advanced circle analysis function using direct field access
#[allow(dead_code)]
pub(crate) fn analyze_circle(circle: &Circle) -> String {
    let area = circle.area();
    let circumference = circle.circumference();
    let center_distance = circle.center.distance_to_origin();
    
    let category = if circle.radius < 1.0 {
        "Small"
    } else if circle.radius < 5.0 {
        "Medium"  
    } else {
        "Large"
    };
    
    let position = if center_distance < f64::EPSILON {
        "at origin"
    } else if center_distance < circle.radius {
        "near origin"
    } else {
        "away from origin"
    };
    
    format!("Circle Analysis: {} circle {} (center=({:.2},{:.2}), r={:.2}, A={:.2}, C={:.2})",
            category, position, circle.center.x, circle.center.y, 
            circle.radius, area, circumference)
}

// Function to check circle intersections using direct field access
#[allow(dead_code)]
pub(crate) fn circles_intersect(c1: &Circle, c2: &Circle) -> bool {
    let center_distance = c1.center.distance(&c2.center);
    let radius_sum = c1.radius + c2.radius;
    let radius_diff = (c1.radius - c2.radius).abs();
    
    // Circles intersect if center distance is between radius difference and radius sum
    center_distance >= radius_diff && center_distance <= radius_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_field_access_in_module() {
        let circle = Circle::new(Point::new(3.0, 4.0), 5.0);
        
        // We can access Circle fields and Point fields directly
        assert_eq!(circle.center.x, 3.0);
        assert_eq!(circle.center.y, 4.0);
        assert_eq!(circle.radius, 5.0);
        
        // Module helper functions can access all fields
        let debug_info = circle_debug_info(&circle);
        assert!(debug_info.contains("center=(3.000,4.000)"));
        assert!(debug_info.contains("radius=5.000"));
    }

    #[test]
    fn test_display_uses_direct_field_access() {
        let circle = Circle::new(Point::new(1.234, 5.678), 2.345);
        
        // Display uses direct field access (no getter calls)
        let display_string = format!("{}", circle);
        assert!(display_string.contains("1.23"));
        assert!(display_string.contains("5.68"));
        assert!(display_string.contains("2.35"));
        
        // Alternate format with calculations
        let alt_format = format!("{:#}", circle);
        assert!(alt_format.contains("center: Point(1.23, 5.68)"));
        assert!(alt_format.contains("radius: 2.35"));
        assert!(alt_format.contains("area:"));
        assert!(alt_format.contains("circumference:"));
        
        // Custom precision
        let precise_format = format!("{:.4}", circle);
        assert!(precise_format.contains("1.2340"));
        assert!(precise_format.contains("5.6780"));
        assert!(precise_format.contains("2.3450"));
    }

    #[test]
    fn test_point_at_angle_with_direct_access() {
        let circle = Circle::new(Point::new(2.0, 3.0), 4.0);
        
        // point_at_angle uses direct field access to center coordinates
        let p0 = circle.point_at_angle(0.0);
        assert!((p0.x - (2.0 + 4.0)).abs() < 1e-10);  // center.x + radius
        assert!((p0.y - 3.0).abs() < 1e-10);          // center.y
        
        let p90 = circle.point_at_angle(std::f64::consts::PI / 2.0);
        assert!((p90.x - 2.0).abs() < 1e-10);         // center.x
        assert!((p90.y - (3.0 + 4.0)).abs() < 1e-10); // center.y + radius
    }

    #[test]
    fn test_bounding_box_with_direct_access() {
        let circle = Circle::new(Point::new(5.0, 10.0), 3.0);
        let (min_point, max_point) = circle.bounding_box();
        
        // Bounding box calculation uses direct field access
        assert_eq!(min_point.x, 2.0);  // center.x - radius
        assert_eq!(min_point.y, 7.0);  // center.y - radius
        assert_eq!(max_point.x, 8.0);  // center.x + radius
        assert_eq!(max_point.y, 13.0); // center.y + radius
    }

    #[test]
    fn test_debug_format_shows_all_fields() {
        let circle = Circle::new(Point::new(1.5, 2.5), 3.5);
        let debug_string = format!("{:?}", circle);
        
        // Debug format shows direct field access
        assert!(debug_string.contains("center.x: 1.5"));
        assert!(debug_string.contains("center.y: 2.5"));
        assert!(debug_string.contains("radius: 3.5"));
        assert!(debug_string.contains("diameter:"));
        assert!(debug_string.contains("area:"));
        assert!(debug_string.contains("circumference:"));
    }

    #[test]
    fn test_module_helper_functions() {
        let circle = Circle::new(Point::new(0.0, 0.0), 2.0);
        
        let debug_info = circle_debug_info(&circle);
        assert!(debug_info.contains("center=(0.000,0.000)"));
        assert!(debug_info.contains("radius=2.000"));
        
        let analysis = analyze_circle(&circle);
        assert!(analysis.contains("Medium circle"));
        assert!(analysis.contains("at origin"));
        assert!(analysis.contains("center=(0.00,0.00)"));
        assert!(analysis.contains("r=2.00"));
    }

    #[test]
    fn test_circle_intersection_function() {
        let c1 = Circle::new(Point::new(0.0, 0.0), 3.0);
        let c2 = Circle::new(Point::new(4.0, 0.0), 2.0);  // Distance = 4, radii = 3+2=5, |3-2|=1
        let c3 = Circle::new(Point::new(10.0, 0.0), 1.0); // Too far apart
        
        // Uses direct field access for calculations
        assert!(circles_intersect(&c1, &c2));  // Distance 4 is between 1 and 5
        assert!(!circles_intersect(&c1, &c3)); // Distance 10 is greater than 3+1=4
    }

    #[test]
    fn test_field_modification() {
        let mut circle = Circle::new(Point::new(1.0, 2.0), 3.0);
        
        // Direct field modification within module
        circle.center.x = 10.0;
        circle.center.y = 20.0;
        circle.radius = 30.0;
        
        assert_eq!(circle.center.x, 10.0);
        assert_eq!(circle.center.y, 20.0);
        assert_eq!(circle.radius, 30.0);
    }

    #[test]
    fn test_public_getters_still_work() {
        let circle = Circle::new(Point::new(7.0, 8.0), 9.0);
        
        // Public getters still exist for external access
        assert_eq!(circle.center().x(), 7.0);
        assert_eq!(circle.center().y(), 8.0);
        assert_eq!(circle.radius(), 9.0);
        
        // But we can also access directly within module
        assert_eq!(circle.center.x, 7.0);
        assert_eq!(circle.center.y, 8.0);
        assert_eq!(circle.radius, 9.0);
    }

    #[test]
    fn test_factory_methods_with_direct_access() {
        let unit = Circle::unit_circle();
        let at_origin = Circle::at_origin(5.0);
        
        // Verify using direct field access
        assert_eq!(unit.center.x, 0.0);
        assert_eq!(unit.center.y, 0.0);
        assert_eq!(unit.radius, 1.0);
        
        assert_eq!(at_origin.center.x, 0.0);
        assert_eq!(at_origin.center.y, 0.0);
        assert_eq!(at_origin.radius, 5.0);
    }

    #[test]
    fn test_mathematical_functions_accuracy() {
        let circle = Circle::new(Point::new(0.0, 0.0), 2.0);
        
        assert_eq!(circle.diameter(), 4.0);
        assert_eq!(circle.area(), std::f64::consts::PI * 4.0);
        assert_eq!(circle.circumference(), 4.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_scientific_notation_format() {
        let circle = Circle::new(
            Point::new(1234.567, 0.0001234),
            9876.543
        );
        
        let sci_format = format!("{:e}", circle);
        assert!(sci_format.contains("e"));
        // Scientific notation format uses direct field access
    }
}
