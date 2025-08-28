// Containers Namespace Module - paul_lopez::containers
// ===================================================
// 
// This module represents the Containers namespace, equivalent to:
// 
// C++:
//   namespace PaulLopez {
//       namespace Containers {
//           template<typename T>
//           class Array;
//       }
//   }
// 
// Rust:
//   mod paul_lopez {
//       pub mod containers {
//           pub struct Array;
//       }
//   }

// Declare individual container modules
mod array;

// Re-export all containers to make them accessible from this module
pub use array::Array;

// Container-specific utilities and constants
pub const DEFAULT_CAPACITY: usize = 10;

/// Container module information
pub fn module_info() -> String {
    format!("Containers module - Data structure classes for storage and organization")
}

/// Get default capacity for containers
pub fn default_capacity() -> usize {
    DEFAULT_CAPACITY
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paul_lopez::cad::Point; // Cross-module import

    #[test]
    fn test_containers_accessible() {
        // Test that container classes are accessible through re-exports
        let _array = Array::with_size(5);
    }

    #[test]
    fn test_cross_module_usage() {
        // Test that containers can work with CAD types
        let mut array = Array::with_size(3);
        let point = Point::new(1.0, 2.0);
        array[0] = point;
        assert_eq!(array[0], point);
    }

    #[test]
    fn test_container_utilities() {
        assert_eq!(default_capacity(), 10);
        let info = module_info();
        assert!(info.contains("Containers"));
    }
}
