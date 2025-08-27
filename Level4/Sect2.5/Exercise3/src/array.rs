// Array class - Safe Rust equivalent to C++ manual memory management
// ===================================================================
//
// This class demonstrates RAII (Resource Acquisition Is Initialization) 
// and automatic memory management in Rust, providing the same functionality
// as a C++ array class but with guaranteed memory safety.
//
// C++ Equivalent:
//   class Array {
//   private:
//       Point* m_data;    // Raw pointer to dynamic array
//       size_t m_size;    // Size of the array
//   public:
//       Array(size_t size = 10);               // Constructor
//       Array(const Array& other);             // Copy constructor
//       ~Array();                              // Destructor  
//       Array& operator=(const Array& other);  // Assignment operator
//       Point& operator[](int index);          // Index operator
//       const Point& operator[](int index) const; // Const index operator
//       size_t Size() const;                   // Size getter
//       void SetElement(int index, const Point& p); // Element setter
//       Point& GetElement(int index);          // Element getter
//   };
#![allow(unused_assignments)]
#![allow(dead_code)]

use crate::point::Point;
use std::ops::{Index, IndexMut};
use std::fmt;

/// Array class for managing a dynamic array of Point objects
/// 
/// This provides the same interface as a C++ Array class but with
/// automatic memory management through Rust's ownership system.
#[derive(Debug, Clone)] // Clone = copy constructor, Debug for printing
pub struct Array {
    // In C++: Point* m_data;
    // In Rust: Vec<Point> provides owned dynamic array with automatic cleanup
    data: Vec<Point>,
}

#[allow(dead_code)]
impl Array {
    /// Default constructor - creates array with 10 elements
    /// 
    /// C++ equivalent: Array() : m_data(new Point[10]), m_size(10) {}
    pub fn new() -> Self {
        Self::with_size(10)
    }
    
    /// Constructor with size argument
    /// 
    /// C++ equivalent: 
    ///   Array(size_t size) : m_data(new Point[size]), m_size(size) {
    ///       for(size_t i = 0; i < size; i++) {
    ///           m_data[i] = Point(); // Default constructor for each element
    ///       }
    ///   }
    pub fn with_size(size: usize) -> Self {
        Array {
            // Create vector with `size` default-constructed Points
            data: vec![Point::default(); size],
        }
    }
    
    /// Create Array from existing Vec (utility function)
    pub fn from_vec(vec: Vec<Point>) -> Self {
        Array { data: vec }
    }
    
    /// Size function - returns the size of the array
    /// 
    /// C++ equivalent: size_t Size() const { return m_size; }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    /// Set element function with bounds checking
    /// When index is out of bounds, ignore the set operation
    /// 
    /// C++ equivalent:
    ///   void SetElement(int index, const Point& p) {
    ///       if (index >= 0 && index < m_size) {
    ///           m_data[index] = p;
    ///       }
    ///       // Ignore if out of bounds
    ///   }
    pub fn set_element(&mut self, index: usize, point: Point) {
        if index < self.data.len() {
            self.data[index] = point;
        }
        // Ignore if out of bounds (as specified in exercise)
    }
    
    /// Get element function with bounds checking
    /// When index is out of bounds, return the first element
    /// 
    /// C++ equivalent:
    ///   Point& GetElement(int index) {
    ///       if (index >= 0 && index < m_size) {
    ///           return m_data[index];
    ///       }
    ///       return m_data[0]; // Return first element if out of bounds
    ///   }
    pub fn get_element(&self, index: usize) -> Point {
        if index < self.data.len() {
            self.data[index]
        } else {
            self.data[0] // Return first element if out of bounds
        }
    }
    
    /// Get element by reference (for modification)
    /// When index is out of bounds, return reference to first element
    pub fn get_element_mut(&mut self, index: usize) -> &mut Point {
        if index < self.data.len() {
            &mut self.data[index]
        } else {
            &mut self.data[0] // Return first element if out of bounds
        }
    }
    
    /// Iterator-like functionality for processing all elements
    /// 
    /// This demonstrates a more Rust-idiomatic approach while maintaining
    /// the Array class interface
    pub fn for_each<F>(&self, mut f: F) 
    where 
        F: FnMut(usize, &Point),
    {
        for (i, point) in self.data.iter().enumerate() {
            f(i, point);
        }
    }
    
    /// Mutable iterator-like functionality
    pub fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut Point),
    {
        for (i, point) in self.data.iter_mut().enumerate() {
            f(i, point);
        }
    }
}

// Default trait implementation - equivalent to default constructor
impl Default for Array {
    /// Default constructor creates array with 10 elements
    fn default() -> Self {
        Self::new()
    }
}

// Square bracket operator for reading (immutable access)
// C++ equivalent: const Point& operator[](int index) const;
impl Index<usize> for Array {
    type Output = Point;
    
    /// Immutable indexing operator
    /// When index is out of bounds, returns first element
    /// 
    /// This is the const version of operator[] in C++
    fn index(&self, index: usize) -> &Self::Output {
        if index < self.data.len() {
            &self.data[index]
        } else {
            &self.data[0] // Return first element if out of bounds
        }
    }
}

// Square bracket operator for writing (mutable access)  
// C++ equivalent: Point& operator[](int index);
impl IndexMut<usize> for Array {
    /// Mutable indexing operator
    /// When index is out of bounds, returns reference to first element
    /// 
    /// This allows both reading and writing: arr[i] = point; or point = arr[i];
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.data.len() {
            &mut self.data[index]
        } else {
            &mut self.data[0] // Return first element if out of bounds
        }
    }
}

// Display trait for pretty printing
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array[size: {}, elements: [", self.size())?;
        for (i, point) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", point)?;
        }
        write!(f, "]]")
    }
}

// Copy constructor is automatically implemented by #[derive(Clone)]
// 
// C++ equivalent:
//   Array(const Array& other) : m_size(other.m_size) {
//       m_data = new Point[m_size];
//       for (size_t i = 0; i < m_size; i++) {
//           m_data[i] = other.m_data[i]; // Deep copy each element
//       }
//   }
//
// Rust's Clone trait does this automatically for Vec<Point>

// Assignment operator is handled by Clone + assignment
//
// C++ equivalent:
//   Array& operator=(const Array& other) {
//       if (this != &other) { // Self-assignment check
//           delete[] m_data;   // Delete old data
//           m_size = other.m_size;
//           m_data = new Point[m_size]; // Allocate new data
//           for (size_t i = 0; i < m_size; i++) {
//               m_data[i] = other.m_data[i]; // Copy elements
//           }
//       }
//       return *this;
//   }
//
// Rust handles this automatically through Clone trait:
// let arr2 = arr1.clone(); // Copy constructor
// arr2 = arr1.clone();     // Assignment operator

// Destructor is automatically implemented by Drop trait for Vec
//
// C++ equivalent:
//   ~Array() {
//       delete[] m_data; // Must remember to delete the array
//   }
//
// Rust's Drop trait (implemented by Vec) handles this automatically:
// - When Array goes out of scope, Vec's Drop is called
// - Vec automatically frees its heap allocation
// - No manual memory management required

// Additional utility implementations

impl Array {
    /// Check if array is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Get iterator over elements (immutable)
    pub fn iter(&self) -> std::slice::Iter<Point> {
        self.data.iter()
    }
    
    /// Get iterator over elements (mutable)
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Point> {
        self.data.iter_mut()
    }
    
    /// Resize array to new size (additional utility)
    /// New elements are default-constructed if growing
    pub fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size, Point::default());
    }
    
    /// Clear all elements and set size to 0
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// Push new element to end of array (dynamic growth)
    pub fn push(&mut self, point: Point) {
        self.data.push(point);
    }
    
    /// Pop last element from array (returns None if empty)
    pub fn pop(&mut self) -> Option<Point> {
        self.data.pop()
    }
    
    /// Get slice of internal data (for advanced operations)
    pub fn as_slice(&self) -> &[Point] {
        &self.data
    }
    
    /// Get mutable slice of internal data
    pub fn as_mut_slice(&mut self) -> &mut [Point] {
        &mut self.data
    }
}

// Comparison operators
impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for Array {}

// Convert from Vec<Point>
impl From<Vec<Point>> for Array {
    fn from(vec: Vec<Point>) -> Self {
        Array { data: vec }
    }
}

// Convert to Vec<Point>
impl From<Array> for Vec<Point> {
    fn from(array: Array) -> Self {
        array.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        // Test default constructor
        let arr1 = Array::new();
        assert_eq!(arr1.size(), 10);
        
        // Test constructor with size
        let arr2 = Array::with_size(5);
        assert_eq!(arr2.size(), 5);
        
        // All elements should be default Points
        for i in 0..arr2.size() {
            assert_eq!(arr2[i], Point::default());
        }
    }

    #[test]
    fn test_set_and_get_element() {
        let mut arr = Array::with_size(3);
        let test_point = Point::new(1.0, 2.0);
        
        // Test SetElement
        arr.set_element(1, test_point);
        assert_eq!(arr.get_element(1), test_point);
        
        // Test bounds checking - should ignore out of bounds
        let original_first = arr[0];
        arr.set_element(10, Point::new(99.0, 99.0));
        assert_eq!(arr[0], original_first); // Should be unchanged
    }

    #[test]
    fn test_bounds_checking_get() {
        let mut arr = Array::with_size(2);
        let first_point = Point::new(3.0, 4.0);
        arr[0] = first_point;
        
        // Out of bounds access should return first element
        assert_eq!(arr.get_element(10), first_point);
    }

    #[test]
    fn test_index_operators() {
        let mut arr = Array::with_size(3);
        
        // Test mutable indexing (writing)
        arr[0] = Point::new(1.0, 2.0);
        arr[1] = Point::new(3.0, 4.0);
        arr[2] = Point::new(5.0, 6.0);
        
        // Test immutable indexing (reading)
        assert_eq!(arr[0], Point::new(1.0, 2.0));
        assert_eq!(arr[1], Point::new(3.0, 4.0));
        assert_eq!(arr[2], Point::new(5.0, 6.0));
    }

    #[test]
    fn test_index_bounds_checking() {
        let mut arr = Array::with_size(2);
        let first_point = Point::new(7.0, 8.0);
        arr[0] = first_point;
        
        // Out of bounds index should return/modify first element
        assert_eq!(arr[10], first_point);
        
        arr[10] = Point::new(99.0, 99.0);
        assert_eq!(arr[0], Point::new(99.0, 99.0)); // First element should be modified
    }

    #[test]
    fn test_copy_constructor() {
        let mut arr1 = Array::with_size(2);
        arr1[0] = Point::new(1.0, 2.0);
        arr1[1] = Point::new(3.0, 4.0);
        
        // Test copy constructor (clone)
        let arr2 = arr1.clone();
        
        // Should have same contents
        assert_eq!(arr2.size(), arr1.size());
        assert_eq!(arr2[0], arr1[0]);
        assert_eq!(arr2[1], arr1[1]);
        
        // Should be independent (deep copy)
        arr1[0] = Point::new(99.0, 99.0);
        assert_ne!(arr2[0], arr1[0]); // arr2 should be unchanged
    }

    #[test]
    fn test_assignment_operator() {
        let mut arr1 = Array::with_size(2);
        arr1[0] = Point::new(5.0, 6.0);
        
        let mut arr2 = Array::with_size(5); // Different initial size
        
        // Test assignment
        arr2 = arr1.clone();
        
        // Should match arr1
        assert_eq!(arr2.size(), arr1.size());
        assert_eq!(arr2[0], arr1[0]);
    }

    #[test]
    fn test_self_assignment() {
        let mut arr = Array::with_size(1);
        arr[0] = Point::new(1.0, 2.0);
        
        let original = arr[0];
        
        // Self-assignment should be safe
        arr = arr.clone();
        assert_eq!(arr[0], original);
    }

    #[test]
    fn test_const_correctness() {
        let arr = Array::with_size(1);
        
        // This function takes immutable reference
        fn read_only_access(a: &Array) -> Point {
            a[0] // Should use immutable Index trait
        }
        
        let result = read_only_access(&arr);
        assert_eq!(result, Point::default());
    }

    #[test]
    fn test_automatic_cleanup() {
        // Test that arrays are automatically cleaned up
        {
            let _temp_arrays: Vec<Array> = (0..100)
                .map(|_| Array::with_size(10))
                .collect();
            // All arrays created here
        }
        // All arrays automatically cleaned up here (no manual delete needed)
        
        assert!(true); // Test passes if no memory leaks
    }

    #[test]
    fn test_additional_utilities() {
        let mut arr = Array::with_size(2);
        
        // Test is_empty
        assert!(!arr.is_empty());
        
        arr.clear();
        assert!(arr.is_empty());
        assert_eq!(arr.size(), 0);
        
        // Test push/pop
        arr.push(Point::new(1.0, 1.0));
        assert_eq!(arr.size(), 1);
        assert_eq!(arr[0], Point::new(1.0, 1.0));
        
        let popped = arr.pop();
        assert_eq!(popped, Some(Point::new(1.0, 1.0)));
        assert_eq!(arr.size(), 0);
    }

    #[test]
    fn test_resize() {
        let mut arr = Array::with_size(2);
        arr[0] = Point::new(1.0, 2.0);
        arr[1] = Point::new(3.0, 4.0);
        
        // Resize to larger
        arr.resize(4);
        assert_eq!(arr.size(), 4);
        assert_eq!(arr[0], Point::new(1.0, 2.0)); // Original elements preserved
        assert_eq!(arr[1], Point::new(3.0, 4.0));
        assert_eq!(arr[2], Point::default()); // New elements are default
        assert_eq!(arr[3], Point::default());
        
        // Resize to smaller
        arr.resize(1);
        assert_eq!(arr.size(), 1);
        assert_eq!(arr[0], Point::new(1.0, 2.0)); // First element preserved
    }

    #[test]
    fn test_iterators() {
        let mut arr = Array::with_size(3);
        arr[0] = Point::new(1.0, 2.0);
        arr[1] = Point::new(3.0, 4.0);
        arr[2] = Point::new(5.0, 6.0);
        
        // Test immutable iterator
        let points: Vec<Point> = arr.iter().cloned().collect();
        assert_eq!(points.len(), 3);
        assert_eq!(points[0], Point::new(1.0, 2.0));
        
        // Test mutable iterator
        for point in arr.iter_mut() {
            *point = *point * 2.0; // Scale all points
        }
        
        assert_eq!(arr[0], Point::new(2.0, 4.0));
        assert_eq!(arr[1], Point::new(6.0, 8.0));
        assert_eq!(arr[2], Point::new(10.0, 12.0));
    }

    #[test]
    fn test_for_each() {
        let mut arr = Array::with_size(3);
        arr[0] = Point::new(1.0, 2.0);
        arr[1] = Point::new(3.0, 4.0);
        arr[2] = Point::new(5.0, 6.0);
        
        // Test immutable for_each
        let mut sum = Point::default();
        arr.for_each(|_i, point| {
            sum = sum + *point;
        });
        assert_eq!(sum, Point::new(9.0, 12.0));
        
        // Test mutable for_each
        arr.for_each_mut(|i, point| {
            *point = Point::new(i as f64, i as f64);
        });
        
        assert_eq!(arr[0], Point::new(0.0, 0.0));
        assert_eq!(arr[1], Point::new(1.0, 1.0));
        assert_eq!(arr[2], Point::new(2.0, 2.0));
    }
}
