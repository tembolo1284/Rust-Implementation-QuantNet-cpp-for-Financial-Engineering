#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

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
    
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    
    #[test]
    fn test_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p2.distance_from_origin(), 5.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }
}
