use std::fmt;

#[derive(Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    // ===== APPROACH 1: Different Names (Recommended) =====
    // This is the most idiomatic Rust approach
    
    // Getters with clear names
    pub fn x(&self) -> f64 {
        self.x
    }
    
    pub fn y(&self) -> f64 {
        self.y
    }
    
    // Setters with clear names
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    
    // Distance functions with different names
    pub fn distance_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    // ===== APPROACH 2: Optional Parameters =====
    // Single function with Option parameter
    
    pub fn distance(&self, target: Option<&Point>) -> f64 {
        match target {
            None => self.distance_origin(),
            Some(point) => self.distance_to(point),
        }
    }
    
    // ===== APPROACH 3: Method Chaining =====
    // Returns &mut self for chaining
    
    pub fn with_x(&mut self, x: f64) -> &mut Self {
        self.x = x;
        self
    }
    
    pub fn with_y(&mut self, y: f64) -> &mut Self {
        self.y = y;
        self
    }
    
    // ===== What we CAN'T do in Rust =====
    // The following would NOT compile:
    
    // pub fn x(&self) -> f64 { self.x }           // Already defined!
    // pub fn x(&mut self, x: f64) { self.x = x }  // ERROR: duplicate definition
    
    // pub fn distance(&self) -> f64 { ... }       // Already defined!
    // pub fn distance(&self, p: &Point) -> f64    // ERROR: duplicate definition
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:.2}, {:.2})", self.x, self.y)
    }
}
