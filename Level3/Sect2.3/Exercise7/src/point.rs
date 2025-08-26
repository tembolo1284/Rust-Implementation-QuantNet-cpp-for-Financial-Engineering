use std::fmt;

#[derive(Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Point {
    // Constructor - often worth inlining for small structs
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    // GETTERS - Using #[inline] hint (like C++ normal inline)
    // C++: inline double GetX() const { return m_x; }
    // These are defined "outside" conceptually (like normal inline)
    
    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }
    
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }
    
    // SETTERS - Using #[inline(always)] (like C++ default inline)
    // C++: void SetX(double x) { m_x = x; }  // inside class
    // Force inlining like methods defined inside C++ class
    
    #[inline(always)]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    
    #[inline(always)]
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    
    // Other methods that benefit from inlining
    
    #[inline]
    pub fn distance_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    #[inline]
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    // ToString - probably NOT worth inlining (allocates String)
    // No inline attribute - let compiler decide
    pub fn to_string(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

// Default - might be worth inlining
impl Default for Point {
    #[inline]
    fn default() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}

impl fmt::Display for Point {
    // Display is usually not inlined (formatting is complex)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({:.2}, {:.2})", self.x, self.y)
    }
}

// Example of preventing inlining for comparison
#[allow(dead_code)]
impl Point {
    // This function will NEVER be inlined
    #[inline(never)]
    pub fn complex_calculation(&self) -> f64 {
        // Imagine this is a complex function we don't want inlined
        let mut result = 0.0;
        for i in 0..100 {
            result += (self.x.powi(i) + self.y.powi(i)).sqrt();
        }
        result
    }
}

// Note: In C++, inline functions must be in header files
// In Rust, everything is in the module file anyway
// The compiler handles the details of where code goes
