// Array class in Containers namespace - paul_lopez::containers::Array
// ====================================================================

// Full class name including namespace for Point used in Array class
// C++: using PaulLopez::CAD::Point; (but we can use just CAD::Point since we're in PaulLopez)
// Rust: Use full path or import from crate root
use crate::paul_lopez::cad::Point; // Full namespace path as required by exercise

use std::ops::{Index, IndexMut};
use std::fmt;

/// Array container class for storing Point objects
/// 
/// This demonstrates cross-module usage - a container from the Containers
/// namespace storing objects from the CAD namespace.
/// Located in the paul_lopez::containers namespace
#[derive(Debug, Clone)]
pub struct Array {
    data: Vec<Point>, // Using Point from paul_lopez::cad namespace
}

impl Array {
    /// Create a new array with default size (10 elements)
    pub fn new() -> Self {
        Self::with_size(super::DEFAULT_CAPACITY)
    }
    
    /// Create an array with specified size
    pub fn with_size(size: usize) -> Self {
        Array {
            // Initialize with default Points from CAD namespace
            data: vec![Point::default(); size],
        }
    }
    
    /// Create array from existing vector of Points
    pub fn from_vec(points: Vec<Point>) -> Self {
        Array { data: points }
    }
    
    /// Get the size of the array
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    /// Set element at given index with bounds checking
    pub fn set_element(&mut self, index: usize, point: Point) {
        if index < self.data.len() {
            self.data[index] = point;
        }
        // Ignore if out of bounds (as per exercise specification)
    }
    
    /// Get element at given index with bounds checking
    /// Returns first element if out of bounds
    pub fn get_element(&self, index: usize) -> Point {
        if index < self.data.len() {
            self.data[index]
        } else {
            self.data[0] // Return first element if out of bounds
        }
    }
    
    /// Check if array is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Clear all elements
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// Add element to end of array
    pub fn push(&mut self, point: Point) {
        self.data.push(point);
    }
    
    /// Remove and return last element
    pub fn pop(&mut self) -> Option<Point> {
        self.data.pop()
    }
    
    /// Resize array to new size
    pub fn resize(&mut self, new_size: usize) {
        self.data.resize(new_size, Point::default());
    }
    
    /// Get iterator over points
    pub fn iter(&self) -> std::slice::Iter<Point> {
        self.data.iter()
    }
    
    /// Get mutable iterator over points
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Point> {
        self.data.iter_mut()
    }
    
    /// Apply function to each element
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(usize, &Point),
    {
        for (i, point) in self.data.iter().enumerate() {
            f(i, point);
        }
    }
    
    /// Apply mutable function to each element
    pub fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut Point),
    {
        for (i, point) in self.data.iter_mut().enumerate() {
            f(i, point);
        }
    }
    
    /// Calculate total distance traveled through all points in order
    pub fn total_path_distance(&self) -> f64 {
        if self.data.len() < 2 {
            return 0.0;
        }
        
        self.data.windows(2)
            .map(|window| window[0].distance(&window[1]))
            .sum()
    }
    
    /// Find the point farthest from origin
    pub fn farthest_from_origin(&self) -> Option<(usize, Point)> {
        self.data.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.distance_to_origin()
                    .partial_cmp(&b.distance_to_origin())
                    .unwrap()
            })
            .map(|(i, &p)| (i, p))
    }
    
    /// Get centroid (average) of all points
    pub fn centroid(&self) -> Point {
        if self.data.is_empty() {
            return Point::default();
        }
        
        let sum = self.data.iter()
            .fold(Point::new(0.0, 0.0), |acc, &p| acc + p);
        
        sum * (1.0 / self.data.len() as f64)
    }
}

impl Default for Array {
    fn default() -> Self {
        Self::new()
    }
}

// Square bracket operator for reading
impl Index<usize> for Array {
    type Output = Point;
    
    fn index(&self, index: usize) -> &Self::Output {
        if index < self.data.len() {
            &self.data[index]
        } else {
            &self.data[0] // Return first element if out of bounds
        }
    }
}

// Square bracket operator for writing
impl IndexMut<usize> for Array {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.data.len() {
            &mut self.data[index]
        } else {
            &mut self.data[0] // Return first element if out of bounds
        }
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array[size: {}, points: [", self.size())?;
        for (i, point) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", point)?;
        }
        write!(f, "]]")
    }
}

// Equality comparison
impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for Array {}

// Convert from Vec<Point>
impl From<Vec<Point>> for Array {
    fn from(vec: Vec<Point>) -> Self {
        Array::from_vec(vec)
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
    // Note: Point is already imported at the top with full namespace path

    #[test]
    fn test_array_creation() {
        let array1 = Array::new();
        assert_eq!(array1.size(), 10); // Default size
        
        let array2 = Array::with_size(5);
        assert_eq!(array2.size(), 5);
        
        // All elements should be default Points
        for i in 0..array2.size() {
            assert_eq!(array2[i], Point::default());
        }
    }

    #[test]
    fn test_cross_module_usage() {
        // Test that Array can store Point objects from CAD module
        let mut array = Array::with_size(3);
        
        // Create Points using full namespace (demonstrating cross-module access)
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let p3 = Point::new(5.0, 6.0);
        
        // Store CAD Points in Container Array
        array[0] = p1;
        array[1] = p2;
        array[2] = p3;
        
        // Verify storage
        assert_eq!(array[0], p1);
        assert_eq!(array[1], p2);
        assert_eq!(array[2], p3);
    }

    #[test]
    fn test_set_get_element() {
        let mut array = Array::with_size(3);
        let test_point = Point::new(7.0, 8.0);
        
        array.set_element(1, test_point);
        assert_eq!(array.get_element(1), test_point);
        
        // Test bounds checking
        let original_first = array[0];
        array.set_element(10, Point::new(99.0, 99.0)); // Out of bounds
        assert_eq!(array[0], original_first); // Should be unchanged
        
        // Out of bounds get should return first element
        assert_eq!(array.get_element(10), original_first);
    }

    #[test]
    fn test_index_operators() {
        let mut array = Array::with_size(2);
        
        // Test writing
        array[0] = Point::new(1.0, 2.0);
        array[1] = Point::new(3.0, 4.0);
        
        // Test reading
        assert_eq!(array[0], Point::new(1.0, 2.0));
        assert_eq!(array[1], Point::new(3.0, 4.0));
        
        // Test out of bounds (should return first element)
        assert_eq!(array[10], array[0]);
    }

    #[test]
    fn test_geometric_operations() {
        let mut array = Array::with_size(3);
        array[0] = Point::new(0.0, 0.0);
        array[1] = Point::new(3.0, 4.0);
        array[2] = Point::new(6.0, 8.0);
        
        // Test total path distance
        let total_distance = array.total_path_distance();
        assert_eq!(total_distance, 10.0); // 5.0 + 5.0
        
        // Test farthest point
        let (index, point) = array.farthest_from_origin().unwrap();
        assert_eq!(index, 2);
        assert_eq!(point, Point::new(6.0, 8.0));
        
        // Test centroid
        let centroid = array.centroid();
        assert_eq!(centroid, Point::new(3.0, 4.0));
    }

    #[test]
    fn test_iterators() {
        let mut array = Array::with_size(3);
        array[0] = Point::new(1.0, 1.0);
        array[1] = Point::new(2.0, 2.0);
        array[2] = Point::new(3.0, 3.0);
        
        // Test immutable iterator
        let sum: f64 = array.iter()
            .map(|p| p.x() + p.y())
            .sum();
        assert_eq!(sum, 12.0); // (1+1) + (2+2) + (3+3)
        
        // Test mutable iterator
        for point in array.iter_mut() {
            *point = *point * 2.0;
        }
        
        assert_eq!(array[0], Point::new(2.0, 2.0));
        assert_eq!(array[1], Point::new(4.0, 4.0));
        assert_eq!(array[2], Point::new(6.0, 6.0));
    }

    #[test]
    fn test_for_each() {
        let mut array = Array::with_size(3);
        array[0] = Point::new(1.0, 2.0);
        array[1] = Point::new(3.0, 4.0);
        array[2] = Point::new(5.0, 6.0);
        
        // Test immutable for_each
        let mut distances = Vec::new();
        array.for_each(|_i, point| {
            distances.push(point.distance_to_origin());
        });
        
        assert_eq!(distances.len(), 3);
        assert!((distances[0] - (5.0_f64).sqrt()).abs() < 1e-10);
        
        // Test mutable for_each
        array.for_each_mut(|i, point| {
            *point = Point::new(i as f64, i as f64);
        });
        
        assert_eq!(array[0], Point::new(0.0, 0.0));
        assert_eq!(array[1], Point::new(1.0, 1.0));
        assert_eq!(array[2], Point::new(2.0, 2.0));
    }

    #[test]
    fn test_array_operations() {
        let mut array = Array::with_size(2);
        
        // Test push/pop
        array.push(Point::new(1.0, 1.0));
        assert_eq!(array.size(), 3);
        
        let popped = array.pop().unwrap();
        assert_eq!(popped, Point::new(1.0, 1.0));
        assert_eq!(array.size(), 2);
        
        // Test resize
        array.resize(5);
        assert_eq!(array.size(), 5);
        
        // Test clear
        array.clear();
        assert!(array.is_empty());
    }

    #[test]
    fn test_clone_and_equality() {
        let mut array1 = Array::with_size(2);
        array1[0] = Point::new(1.0, 2.0);
        array1[1] = Point::new(3.0, 4.0);
        
        let array2 = array1.clone();
        assert_eq!(array1, array2);
        
        // Modify original
        array1[0] = Point::new(5.0, 6.0);
        assert_ne!(array1, array2); // Should be different now
    }

    #[test]
    fn test_conversions() {
        let points = vec![
            Point::new(1.0, 2.0),
            Point::new(3.0, 4.0),
            Point::new(5.0, 6.0),
        ];
        
        let array = Array::from(points.clone());
        assert_eq!(array.size(), 3);
        assert_eq!(array[0], points[0]);
        
        let back_to_vec: Vec<Point> = array.into();
        assert_eq!(back_to_vec, points);
    }
}
