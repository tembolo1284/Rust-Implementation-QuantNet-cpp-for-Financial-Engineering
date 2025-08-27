# Level 3: Classes and Objects - C++ to Rust Translation

## Overview
Level 3 introduces object-oriented programming concepts from C++ and their Rust equivalents. Since Rust doesn't have traditional classes or inheritance, this level demonstrates how to achieve similar functionality using structs, impl blocks, traits, and composition.

---

## Section 2.2: The Class Concept

### Learning Objectives
- Understand classes vs structs
- Implement constructors and destructors
- Create member functions (methods)
- Handle private/public access control

### Exercise 1: Point Class
**C++ Focus**: Basic class with private members, constructors, destructor, getters/setters  
**Rust Translation**: Struct with impl blocks, Default trait, Drop trait

```rust
pub struct Point {
    x: f64,  // Private by default
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self { /* ... */ }
    pub fn get_x(&self) -> f64 { /* ... */ }
    pub fn set_x(&mut self, x: f64) { /* ... */ }
}
```

**Key Concepts**:
- Fields are private by default in Rust
- No header/source file separation
- `Self` refers to the struct type
- Methods in `impl` blocks

### Exercise 2: Distance Functions
**C++ Focus**: Member functions for distance calculations  
**Rust Translation**: Methods that take `&self`

```rust
impl Point {
    pub fn distance_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
}
```

**Key Difference**: Rust passes `other` by reference (`&Point`) to avoid copying, unlike C++ which would copy by default.

---

## Section 2.3: Improving Efficiency

### Exercise 1: Extra Constructors
**C++ Focus**: Multiple constructors, copy constructor, tracking object lifecycle  
**Rust Translation**: Multiple `new` methods, Clone trait, Drop trait

```rust
impl Point {
    pub fn new(x: f64, y: f64) -> Self { /* ... */ }
    pub fn from_x(x: f64) -> Self { /* ... */ }
    pub fn default() -> Self { /* ... */ }
}

#[derive(Clone)]  // Automatic copy constructor
struct Point { /* ... */ }
```

**Key Learning**:
- Rust moves by default, copies must be explicit with `.clone()`
- No implicit copy constructor - must derive or implement Clone
- Drop trait for destructor behavior (rarely needed)

### Exercise 2: Pass by Reference
**C++ Focus**: Const reference parameters to avoid copies  
**Rust Translation**: References are immutable by default

| C++ | Rust |
|-----|------|
| `double Distance(const Point& p)` | `fn distance(&self, p: &Point)` |
| `void Modify(Point& p)` | `fn modify(&mut self, p: &mut Point)` |

**Key Insight**: Rust references are always "const" unless explicitly `&mut`.

### Exercise 3: Function Overloading
**C++ Focus**: Same function name with different parameters  
**Rust Translation**: NOT SUPPORTED - use different names or patterns

```rust
// C++: Can't do this in Rust
// double Distance();
// double Distance(Point& p);

// Rust alternatives:
fn distance_origin(&self) -> f64 { /* ... */ }
fn distance_to(&self, other: &Point) -> f64 { /* ... */ }

// Or use Option:
fn distance(&self, target: Option<&Point>) -> f64 {
    match target {
        None => self.distance_origin(),
        Some(p) => self.distance_to(p),
    }
}
```

### Exercise 4: Const Functions
**C++ Focus**: Mark functions as `const` to call on const objects  
**Rust Translation**: `&self` vs `&mut self` borrowing

```rust
impl Point {
    // Like C++ const function - can call on immutable
    pub fn x(&self) -> f64 { self.x }
    
    // Like C++ non-const - needs mutable
    pub fn set_x(&mut self, x: f64) { self.x = x; }
}

let point = Point::new(1.0, 2.0);      // Immutable
point.x();        // OK - takes &self
point.set_x(5.0); // ERROR - needs mut

let mut point = Point::new(1.0, 2.0);  // Mutable
point.x();        // OK
point.set_x(5.0); // OK
```

### Exercise 5: Line Class
**C++ Focus**: Composition, delegation  
**Rust Translation**: Struct containing other structs

```rust
pub struct Line {
    start_point: Point,  // Composition: HAS-A
    end_point: Point,
}

impl Line {
    pub fn length(&self) -> f64 {
        // Delegation to Point's distance method
        self.start_point.distance(&self.end_point)
    }
}
```

**Key Concepts**:
- Composition over inheritance
- Delegation pattern
- No friend functions needed

### Exercise 6: Circle Class
**C++ Focus**: Another class with composition  
**Rust Translation**: Struct with Point center and radius

```rust
pub struct Circle {
    center_point: Point,
    radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    
    pub fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }
}
```

### Exercise 7: Inline Functions
**C++ Focus**: `inline` keyword for optimization  
**Rust Translation**: `#[inline]` attribute hints

```rust
impl Point {
    #[inline]           // Hint to compiler
    pub fn x(&self) -> f64 { self.x }
    
    #[inline(always)]   // Force inlining
    pub fn set_x(&mut self, x: f64) { self.x = x; }
    
    #[inline(never)]    // Prevent inlining
    pub fn complex_calculation(&self) -> f64 { /* ... */ }
}
```

**Key Differences**:
- C++ requires `inline` in header files
- Rust compiler auto-optimizes
- Attributes are hints, not requirements

---

## Key C++ to Rust Mappings

### Class Structure
| C++ | Rust | Notes |
|-----|------|-------|
| `class` | `struct` | No inheritance in Rust |
| `private:` | (default) | Fields private by default |
| `public:` | `pub` | Explicit public visibility |
| Constructor | `new()` method | Just a convention |
| Destructor | `Drop` trait | Usually automatic |
| Copy constructor | `Clone` trait | Must be explicit |

### Method Types
| C++ | Rust | Purpose |
|-----|------|---------|
| `void Method()` | `fn method(&mut self)` | Modifying method |
| `void Method() const` | `fn method(&self)` | Non-modifying method |
| `static void Method()` | `fn method()` | Associated function |
| Operator overload | Trait implementation | `Display`, `Add`, etc. |

### Memory Management
| C++ Pattern | Rust Equivalent | Benefit |
|-------------|-----------------|---------|
| Manual new/delete | Automatic | No memory leaks |
| Copy constructor | `.clone()` | Explicit cost |
| Pass by value | Move semantics | No hidden copies |
| Pass by const& | Pass by `&` | Always immutable |
| Pass by & | Pass by `&mut` | Explicit mutability |

---

## Design Patterns in Level 3

### Composition Over Inheritance
Rust doesn't have inheritance. Use composition:
```rust
// Instead of: class Line : public Shape
// Use: 
struct Line {
    shape_data: ShapeData,  // Composition
    start: Point,
    end: Point,
}
```

### Builder Pattern
For complex object construction:
```rust
impl Point {
    pub fn new(x: f64, y: f64) -> Self { /* ... */ }
    pub fn with_x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }
}
// Usage: Point::new(0.0, 0.0).with_x(5.0).with_y(10.0)
```

### Delegation
Reuse functionality through delegation:
```rust
impl Line {
    pub fn length(&self) -> f64 {
        self.start.distance(&self.end)  // Delegate to Point
    }
}
```

---

## Common Pitfalls and Solutions

### Pitfall 1: Expecting Function Overloading
**Problem**: Rust doesn't support multiple functions with same name  
**Solution**: Use different names or Option parameters

### Pitfall 2: Forgetting Clone
**Problem**: `let p2 = p1;` moves p1, making it unusable  
**Solution**: Use `let p2 = p1.clone();` or references

### Pitfall 3: Const Correctness Confusion
**Problem**: When to use `&self` vs `&mut self`  
**Solution**: 
- Reading only? Use `&self`
- Modifying? Use `&mut self`
- Creating new? No self parameter

### Pitfall 4: Missing Public Visibility
**Problem**: Forgetting `pub` makes items private to module  
**Solution**: Add `pub` for public API

---

## Testing Your Code

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }
}
```

Run tests:
```bash
cargo test
```

---

## Performance Considerations

### Debug vs Release Builds
```bash
cargo run              # Debug: No optimizations
cargo run --release    # Release: Full optimizations
```

### Inlining
- Small functions: Add `#[inline]`
- Hot paths: Consider `#[inline(always)]`
- Large functions: Let compiler decide

### Zero-Cost Abstractions
- Methods have no runtime overhead
- Traits compile to static dispatch by default
- Generics are monomorphized

---

## Summary of Level 3 Concepts

✅ **Structs Instead of Classes**: Rust uses structs with impl blocks  
✅ **No Inheritance**: Use composition and traits  
✅ **Explicit Mutability**: `&self` vs `&mut self`  
✅ **No Function Overloading**: Use different names  
✅ **Move Semantics**: Ownership transfer by default  
✅ **Explicit Cloning**: No hidden copies  
✅ **Automatic Memory Management**: No manual delete  
✅ **Const by Default**: References are immutable unless `&mut`  

---

## Key Takeaways

1. **Rust's struct + impl pattern is simpler than C++ classes**
   - No header/source split
   - No forward declarations
   - Clear ownership

2. **Safety without overhead**
   - Borrow checker prevents bugs
   - Zero-cost abstractions
   - No runtime penalty

3. **Explicit is better than implicit**
   - No hidden copies
   - Clear mutability
   - Visible ownership transfer

4. **Composition encourages better design**
   - More flexible than inheritance
   - Easier to reason about
   - Better encapsulation

