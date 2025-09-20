use rand::Rng;
use std::fmt;

// ShapeData - Contains the common data that would be in a C++ base class
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeData {
    id: i32,
}

#[allow(dead_code)]
impl ShapeData {
    pub fn new() -> Self {
        Self {
            id: rand::thread_rng().gen_range(1000..9999),
        }
    }

    pub fn with_id(id: i32) -> Self {
        Self { id }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    // This represents the "base class" ToString functionality
    pub fn to_string(&self) -> String {
        format!("ID: {}", self.id)
    }
}

// =============================================================================
// SHAPE TRAIT WITH BASE CLASS FUNCTIONALITY
// =============================================================================

#[allow(dead_code)]
pub trait Shape: fmt::Display {
    fn shape_data(&self) -> &ShapeData;
    fn shape_data_mut(&mut self) -> &mut ShapeData;

    fn id(&self) -> i32 {
        self.shape_data().id()
    }

    // Default "base class" implementation - equivalent to C++ Shape::ToString()
    fn base_to_string(&self) -> String {
        self.shape_data().to_string()
    }

    // Virtual function that can be overridden - equivalent to C++ virtual ToString()
    fn to_string(&self) -> String {
        self.base_to_string()  // Default calls base implementation
    }

    fn shape_type(&self) -> &'static str {
        "Unknown Shape"
    }

    // Helper method to explicitly call base functionality (simulates C++ Shape::ToString())
    fn call_base_to_string(&self) -> String {
        Shape::base_to_string(self)
    }

    // Another way to access base functionality
    fn delegate_to_base(&self) -> String {
        self.shape_data().to_string()
    }
}

// ShapeImpl - Concrete implementation
#[derive(Debug, Clone)]
pub struct ShapeImpl {
    shape_data: ShapeData,
}

#[allow(dead_code)]
impl ShapeImpl {
    pub fn new() -> Self {
        println!("  ShapeImpl::new() - Creating base shape");
        Self {
            shape_data: ShapeData::new(),
        }
    }

    pub fn with_id(id: i32) -> Self {
        Self {
            shape_data: ShapeData::with_id(id),
        }
    }
}

#[allow(dead_code)]
impl Shape for ShapeImpl {
    fn shape_data(&self) -> &ShapeData {
        &self.shape_data
    }

    fn shape_data_mut(&mut self) -> &mut ShapeData {
        &mut self.shape_data
    }

    fn to_string(&self) -> String {
        format!("Shape [{}]", self.base_to_string())
    }

    fn shape_type(&self) -> &'static str {
        "Shape"
    }
}

#[allow(dead_code)]
// Implement Display for ShapeImpl
impl fmt::Display for ShapeImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = <Self as Shape>::to_string(self);
        f.write_str(&s)
    }
}

// Drop implementations
impl Drop for ShapeData {
    fn drop(&mut self) {
        println!("    ShapeData::drop() - ID {}", self.id);
    }
}

impl Drop for ShapeImpl {
    fn drop(&mut self) {
        println!("  ShapeImpl::drop()");
    }
}

// Default implementations
impl Default for ShapeData {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ShapeImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_to_string() {
        let shape = ShapeImpl::new();
        let base_result = shape.base_to_string();
        assert!(base_result.contains("ID:"));
    }

    #[test]
    fn test_explicit_base_call() {
        let shape = ShapeImpl::new();
        let base_result = Shape::base_to_string(&shape);
        let direct_result = shape.base_to_string();
        assert_eq!(base_result, direct_result);
    }

    #[test]
    fn test_shape_data() {
        let data = ShapeData::new();
        assert!(data.id() >= 1000 && data.id() <= 9999);
        assert!(data.to_string().contains("ID:"));
    }

    #[test]
    fn test_delegate_to_base() {
        let shape = ShapeImpl::new();
        let delegated = shape.delegate_to_base();
        let direct = shape.shape_data().to_string();
        assert_eq!(delegated, direct);
    }
}
