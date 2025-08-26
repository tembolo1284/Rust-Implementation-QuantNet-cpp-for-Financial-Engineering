use std::fmt;

#[derive(Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Constructor - returns new instance
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    // ===== CONST FUNCTIONS (C++) = &self methods (Rust) =====
    // These can be called on immutable references
    // They do NOT modify the Point
    
    // Getter for x - takes &self (immutable reference)
    // C++: double X() const
    pub fn x(&self) -> f64 {
        self.x
    }
    
    // Getter for y - takes &self (immutable reference)  
    // C++: double Y() const
    pub fn y(&self) -> f64 {
        self.y
    }
    
    // Distance to origin - takes &self (immutable reference)
    // C++: double Distance() const
    pub fn distance_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    // Distance to another point - takes &self (immutable reference)
    // C++: double Distance(const Point& p) const
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    // ToString - takes &self (immutable reference)
    // C++: string ToString() const
    pub fn to_string(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
    
    // ===== NON-CONST FUNCTIONS (C++) = &mut self methods (Rust) =====
    // These require mutable references
    // They CAN modify the Point
    
    // Setter for x - takes &mut self (mutable reference)
    // C++: void SetX(double x) // NOT const
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    
    // Setter for y - takes &mut self (mutable reference)
    // C++: void SetY(double y) // NOT const
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}

// Display trait also takes &self (immutable) - like a const function
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:.2}, {:.2})", self.x, self.y)
    }
}
