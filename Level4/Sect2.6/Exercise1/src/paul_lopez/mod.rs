// Paul Lopez Namespace Module
// ===========================
// 
// This is the root module for the PaulLopez namespace, equivalent to:
// 
// C++:
//   namespace PaulLopez {
//       // Contains CAD and Containers sub-namespaces
//   }
// 
// Rust:
//   mod paul_lopez {
//       pub mod cad;
//       pub mod containers;
//   }
#![allow(unused_imports)]

// Declare sub-modules (sub-namespaces)
pub mod cad;       // PaulLopez::CAD namespace equivalent
pub mod containers; // PaulLopez::Containers namespace equivalent

// Re-export commonly used types at the namespace root level
// This allows access like: paul_lopez::Point instead of paul_lopez::cad::Point
pub use cad::Point;
pub use cad::Line;
pub use cad::Circle;
pub use cad::Shape;
pub use containers::Array;

// Module-level documentation and utilities
pub const AUTHOR: &str = "Paul Lopez";
pub const VERSION: &str = "1.0.0";

/// Get information about this namespace
pub fn namespace_info() -> String {
    format!("PaulLopez namespace v{} - CAD and Container classes", VERSION)
}

/// Demonstrate namespace organization
pub fn list_available_classes() {
    println!("Available classes in PaulLopez namespace:");
    println!("  CAD module:");
    println!("    - Point: 2D geometric point");
    println!("    - Line: Line between two points");
    println!("    - Circle: Circle with center and radius");
    println!("    - Shape: Base geometric shape");
    println!("  Containers module:");
    println!("    - Array: Dynamic array container");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_structure() {
        // Test that we can access classes from both sub-namespaces
        let _point = cad::Point::new(1.0, 2.0);
        let _array = containers::Array::with_size(5);
        
        // Test re-exports work
        let _point2 = Point::new(3.0, 4.0);
        let _array2 = Array::with_size(3);
    }

    #[test]
    fn test_namespace_info() {
        let info = namespace_info();
        assert!(info.contains("PaulLopez"));
        assert!(info.contains("CAD"));
        assert!(info.contains("Container"));
    }
}
