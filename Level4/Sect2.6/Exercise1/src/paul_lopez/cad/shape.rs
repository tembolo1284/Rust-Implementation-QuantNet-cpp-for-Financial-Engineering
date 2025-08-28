// Shape class in CAD namespace - paul_lopez::cad::Shape
// ====================================================
#![allow(dead_code)]

use std::fmt;

/// Base Shape class for all geometric shapes
/// 
/// This represents a generic geometric shape with a name
/// Located in the paul_lopez::cad namespace
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    name: String,
    id: u32,
    visible: bool,
}

// Static counter for generating unique IDs
static mut NEXT_ID: u32 = 1;

impl Shape {
    /// Create a new shape with given name
    pub fn new(name: &str) -> Self {
        let id = unsafe {
            let current_id = NEXT_ID;
            NEXT_ID += 1;
            current_id
        };
        
        Shape {
            name: name.to_string(),
            id,
            visible: true,
        }
    }
    
    // /// Create a default shape
    // pub fn default() -> Self {
    //     Shape::new("Unnamed Shape")
    // }
    
    /// Get the shape name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Set the shape name
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    
    /// Get the shape ID
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Check if the shape is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    /// Set shape visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    /// Show the shape
    pub fn show(&mut self) {
        self.visible = true;
    }
    
    /// Hide the shape
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    /// Get shape description
    pub fn description(&self) -> String {
        format!("Shape '{}' (ID: {}, Visible: {})", 
                self.name, self.id, self.visible)
    }
    
    /// Create a copy of the shape with a new name
    pub fn copy_with_name(&self, new_name: &str) -> Shape {
        let mut copy = self.clone();
        copy.set_name(new_name);
        // Keep the same ID for copies, but in a real system you might want new IDs
        copy
    }
}

impl Default for Shape {
    fn default() -> Self {
        Shape::new("Default Shape")
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.visible {
            write!(f, "Shape[name: '{}', id: {}]", self.name, self.id)
        } else {
            write!(f, "Shape[name: '{}', id: {}, HIDDEN]", self.name, self.id)
        }
    }
}

// Additional shape utilities
impl Shape {
    /// Get the next available ID (for debugging)
    pub fn next_available_id() -> u32 {
        unsafe { NEXT_ID }
    }
    
    /// Reset the ID counter (for testing)
    pub fn reset_id_counter() {
        unsafe { NEXT_ID = 1; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_creation() {
        Shape::reset_id_counter(); // Reset for consistent testing
        
        let shape1 = Shape::new("Triangle");
        assert_eq!(shape1.name(), "Triangle");
        assert_eq!(shape1.id(), 1);
        assert!(shape1.is_visible());
        
        let shape2 = Shape::new("Rectangle");
        assert_eq!(shape2.id(), 2);
    }

    #[test]
    fn test_shape_visibility() {
        let mut shape = Shape::new("Test Shape");
        
        assert!(shape.is_visible());
        
        shape.hide();
        assert!(!shape.is_visible());
        
        shape.show();
        assert!(shape.is_visible());
        
        shape.set_visible(false);
        assert!(!shape.is_visible());
    }

    #[test]
    fn test_shape_name_changes() {
        let mut shape = Shape::new("Original");
        assert_eq!(shape.name(), "Original");
        
        shape.set_name("Modified");
        assert_eq!(shape.name(), "Modified");
    }

    #[test]
    fn test_shape_copy() {
        Shape::reset_id_counter();
        
        let original = Shape::new("Original Shape");
        let copy = original.copy_with_name("Copy Shape");
        
        assert_eq!(copy.name(), "Copy Shape");
        assert_eq!(copy.id(), original.id()); // Same ID in this implementation
        assert_eq!(copy.is_visible(), original.is_visible());
    }

    #[test]
    fn test_shape_description() {
        Shape::reset_id_counter();
        
        let shape = Shape::new("Test");
        let desc = shape.description();
        
        assert!(desc.contains("Test"));
        assert!(desc.contains("ID: 1"));
        assert!(desc.contains("Visible: true"));
    }

    #[test]
    fn test_default_shape() {
        let shape = Shape::default();
        assert_eq!(shape.name(), "Default Shape");
        assert!(shape.is_visible());
    }

    #[test]
    fn test_shape_display() {
        Shape::reset_id_counter();
        
        let mut shape = Shape::new("Display Test");
        let visible_display = format!("{}", shape);
        assert!(visible_display.contains("Display Test"));
        assert!(!visible_display.contains("HIDDEN"));
        
        shape.hide();
        let hidden_display = format!("{}", shape);
        assert!(hidden_display.contains("HIDDEN"));
    }
}
