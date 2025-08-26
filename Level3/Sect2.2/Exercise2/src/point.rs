use std::fmt;

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        println!("Creating Point({}, {})", x, y);
        Point { x, y }
    }
    
    pub fn get_x(&self) -> f64 {
        self.x
    }
    
    pub fn get_y(&self) -> f64 {
        self.y
    }
    
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    
    pub fn to_string(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
    
    // Calculate distance to origin (0, 0)
    // C++: double DistanceOrigin() const
    pub fn distance_origin(&self) -> f64 {
        // Distance = √(x² + y²)
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    // Calculate distance to another point
    // C++: double Distance(Point p) const  
    // Note: We pass by reference (&Point) to avoid copying!
    pub fn distance(&self, other: &Point) -> f64 {
        // Distance = √((x₂-x₁)² + (y₂-y₁)²)
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl Default for Point {
    fn default() -> Self {
        println!("Default constructor called");
        Point { x: 0.0, y: 0.0 }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:.2}, {:.2})", self.x, self.y)
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Dropping Point({}, {})", self.x, self.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.get_x(), 3.0);
        assert_eq!(p.get_y(), 4.0);
    }
    
    #[test]
    fn test_default() {
        let p = Point::default();
        assert_eq!(p.get_x(), 0.0);
        assert_eq!(p.get_y(), 0.0);
    }
    
    #[test]
    fn test_setters() {
        let mut p = Point::default();
        p.set_x(5.0);
        p.set_y(10.0);
        assert_eq!(p.get_x(), 5.0);
        assert_eq!(p.get_y(), 10.0);
    }
    
    #[test]
    fn test_to_string() {
        let p = Point::new(1.5, 2.5);
        assert_eq!(p.to_string(), "Point(1.5, 2.5)");
    }
    
    // NEW TESTS for distance functions
    #[test]
    fn test_distance_origin() {
        let origin = Point::default();
        assert_eq!(origin.distance_origin(), 0.0);
        
        let p1 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance_origin(), 5.0);  // 3-4-5 triangle
        
        let p2 = Point::new(1.0, 0.0);
        assert_eq!(p2.distance_origin(), 1.0);
    }
    
    #[test]
    fn test_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
        
        let p3 = Point::new(1.0, 1.0);
        let p4 = Point::new(1.0, 1.0);
        assert_eq!(p3.distance(&p4), 0.0);  // Same point
        
        let p5 = Point::new(0.0, 0.0);
        let p6 = Point::new(1.0, 1.0);
        let expected = (2.0_f64).sqrt();
        assert!((p5.distance(&p6) - expected).abs() < 1e-10);
    }
}
