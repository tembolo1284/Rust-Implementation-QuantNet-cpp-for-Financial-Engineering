# Level 2: The Preprocessor - From C Macros to Rust

## Overview
Level 2 focuses on the C/C++ preprocessor and its Rust equivalents. This level demonstrates the fundamental differences between C's text-substitution macros and Rust's hygienic macro system, showcasing how Rust achieves the same goals with better safety and maintainability.

---

## The Fundamental Difference

### C/C++ Preprocessor
- **Text substitution** before compilation
- No understanding of language syntax
- Can create subtle bugs
- Powerful but dangerous

### Rust Macro System
- **AST manipulation** during compilation
- Syntax-aware expansion
- Hygienic (no variable capture)
- Type-checked after expansion

---

## Section 1.6: Basic Macros

### Exercise 1: PRINT Macros
**Objective**: Create macros for printing values

#### C++ Implementation
```c
#define PRINT1(x) printf("%d\n", x)
#define PRINT2(x,y) printf("%d, %d\n", x, y)
```

**Problems with C Macros**:
- No type safety
- Format specifier must match type
- Can cause unexpected expansions

#### Rust Implementation
```rust
macro_rules! print1 {
    ($x:expr) => {
        println!("{}", $x);
    };
}

macro_rules! print2 {
    ($x:expr, $y:expr) => {
        println!("{}, {}", $x, $y);
    };
}
```

**Rust Advantages**:
- Type-safe (Display trait)
- Works with any displayable type
- No format specifier needed
- Syntax checking at macro definition

### Exercise 2: MAX Macros
**Objective**: Create macros for finding maximum values

#### C++ Implementation
```c
#define MAX2(x,y) ((x) > (y) ? (x) : (y))
#define MAX3(x,y,z) MAX2(MAX2(x,y), z)
```

**Critical C++ Issues**:

1. **Double Evaluation**
```c
int i = 5, j = 6;
int m = MAX2(i++, j++);  // i and j incremented TWICE!
// Expands to: ((i++) > (j++) ? (i++) : (j++))
```

2. **Operator Precedence**
```c
#define SQUARE(x) x * x
int result = SQUARE(2 + 3);  // Returns 11, not 25!
// Expands to: 2 + 3 * 2 + 3
```

#### Rust Implementation
```rust
macro_rules! max2 {
    ($x:expr, $y:expr) => {{
        let a = $x;  // Evaluate once
        let b = $y;  // Evaluate once
        if a > b { a } else { b }
    }};
}

macro_rules! max3 {
    ($x:expr, $y:expr, $z:expr) => {
        max2!(max2!($x, $y), $z)
    };
}
```

**Rust Solutions**:
- Variables prevent double evaluation
- Block expressions handle precedence
- Actually, prefer generic functions:

```rust
fn max2<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

---

## Section 1.7: Advanced Pointer and Array Operations

### Exercise 3: Array Iteration Patterns
**Objective**: Demonstrate various array traversal methods

#### C++ Pointer Arithmetic
```c
int a[] = {0, 1, 2, 3, 4};
int* p;

// Various iteration patterns
for (p = &a[0]; p <= &a[4]; p++) printf("%d", *p);
for (p = a + 4; p >= a; p--) printf("%d", *p);
for (p = a + 4, i = 0; i <= 4; i++) printf("%d", p[-i]);
```

**C++ Dangers**:
- Buffer overruns
- Pointer arithmetic errors
- No bounds checking

#### Rust Safe Alternatives
```rust
let a = [0, 1, 2, 3, 4];

// Safe forward iteration
for &item in a.iter() {
    print!("{}", item);
}

// Safe reverse iteration
for &item in a.iter().rev() {
    print!("{}", item);
}

// Safe index access (bounds checked)
for i in (0..a.len()).rev() {
    print!("{}", a[i]);
}
```

**Rust Safety Features**:
- Automatic bounds checking
- No pointer arithmetic in safe code
- Iterator patterns prevent off-by-one errors

### Exercise 4: String Arrays
**Objective**: Array of strings for day names

#### C++ Implementation
```c
const char* days[] = {
    "Sunday", "Monday", "Tuesday", "Wednesday",
    "Thursday", "Friday", "Saturday"
};

void print_day(int day_num) {
    if (day_num >= 1 && day_num <= 7) {
        printf("Day %d is a %s\n", day_num, days[day_num - 1]);
    }
}
```

#### Rust Implementation
```rust
const DAYS: [&str; 7] = [
    "Sunday", "Monday", "Tuesday", "Wednesday",
    "Thursday", "Friday", "Saturday"
];

fn print_day(day_num: usize) {
    match day_num {
        1..=7 => println!("Day {} is a {}", day_num, DAYS[day_num - 1]),
        _ => println!("Invalid day number"),
    }
}
```

---

## Include Guards vs Rust Modules

### C++ Include Guards
```c
#ifndef DEFS_H
#define DEFS_H
// ... header content ...
#endif
```

**Purpose**: Prevent multiple inclusion  
**Problems**: 
- Manual process
- Name collisions possible
- Forgotten guards cause compilation errors

### Rust Module System
```rust
// No guards needed!
mod defs;  // Automatically handled by compiler
```

**Advantages**:
- Automatic handling
- No multiple inclusion issues
- Clear dependency graph
- Privacy control

---

## Conditional Compilation

### C++ Approach
```c
#ifdef DEBUG
    #define LOG(x) printf("Debug: %s\n", x)
#else
    #define LOG(x)
#endif
```

### Rust Approach
```rust
#[cfg(debug_assertions)]
macro_rules! log {
    ($x:expr) => { println!("Debug: {}", $x); };
}

#[cfg(not(debug_assertions))]
macro_rules! log {
    ($x:expr) => {};
}
```

**Or using attributes:**
```rust
#[cfg(feature = "logging")]
fn log_message(msg: &str) {
    println!("{}", msg);
}
```

---

## Common Macro Pitfalls and Solutions

### Pitfall 1: Multiple Evaluation

**C++ Problem**:
```c
#define DOUBLE(x) (x) * 2
int y = DOUBLE(expensive_function());  // Called twice!
```

**Rust Solution**:
```rust
macro_rules! double {
    ($x:expr) => {{
        let val = $x;  // Evaluate once
        val * 2
    }};
}
```

### Pitfall 2: Variable Capture

**C++ Problem**:
```c
#define SWAP(a, b) { int temp = a; a = b; b = temp; }
int temp = 5;
SWAP(x, y);  // 'temp' collision!
```

**Rust Solution**:
```rust
macro_rules! swap {
    ($a:expr, $b:expr) => {
        std::mem::swap(&mut $a, &mut $b)
    };
}
// Hygienic - no variable collision
```

### Pitfall 3: Type Safety

**C++ Problem**:
```c
#define MAX(a, b) ((a) > (b) ? (a) : (b))
MAX("hello", "world");  // Compiles but wrong!
```

**Rust Solution**:
```rust
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
// Type-safe with trait bounds
```

---

## Best Practices: When to Use Macros

### Use Macros For:
1. **Domain-specific languages** (DSLs)
2. **Code generation** that can't be done with functions
3. **Conditional compilation**
4. **Variadic arguments** before Rust 1.53

### Prefer Functions For:
1. **Simple computations** (use generics)
2. **Type-safe operations**
3. **Debuggable code**
4. **Public APIs**

---

## Macro Design Patterns

### Pattern 1: Internal Rules
```rust
macro_rules! my_macro {
    // Public interface
    ($x:expr) => {
        my_macro!(@internal $x, 0)
    };
    
    // Internal implementation
    (@internal $x:expr, $count:expr) => {
        // Implementation details
    };
}
```

### Pattern 2: Incremental TT Munching
```rust
macro_rules! count_items {
    () => { 0 };
    ($head:tt $($tail:tt)*) => {
        1 + count_items!($($tail)*)
    };
}
```

### Pattern 3: Callback Pattern
```rust
macro_rules! with_generated {
    ($callback:ident) => {
        $callback!(generated_value);
    };
}
```

---

## Debugging Macros

### C++ Debugging
```c
gcc -E file.c  # See preprocessed output
```

### Rust Debugging
```rust
// Use trace_macros
#![feature(trace_macros)]
trace_macros!(true);
my_macro!(x);
trace_macros!(false);

// Use log_syntax
#![feature(log_syntax)]
macro_rules! debug_macro {
    ($x:expr) => {
        log_syntax!($x);
    };
}

// Cargo expand
cargo install cargo-expand
cargo expand
```

---

## Migration Guide: C Macros to Rust

| C Macro Pattern | Rust Alternative | Benefits |
|-----------------|------------------|----------|
| `#define CONST 42` | `const CONST: i32 = 42;` | Type-safe |
| `#define MAX(a,b)` | Generic function | No double evaluation |
| `#ifdef DEBUG` | `#[cfg(debug_assertions)]` | Integrated with build |
| Include guards | Module system | Automatic |
| Function-like macros | `macro_rules!` or functions | Hygienic |
| Variadic macros | `macro_rules!` with repetition | Pattern matching |

---

## Performance Considerations

### Zero-Cost Abstractions
- Rust macros expand at compile time
- No runtime overhead
- Same performance as hand-written code
- Inline functions often better than macros

### Compile-Time Cost
- Complex macros increase compile time
- Recursive macros can explode
- Consider build time vs. runtime tradeoff
- Use `cargo build --timings` to profile

---

## Common Exercises Summary

| Exercise | C/C++ Focus | Rust Approach | Key Learning |
|----------|-------------|---------------|--------------|
| PRINT macros | `printf` wrappers | `println!` with Display | Type safety |
| MAX macros | Comparison macros | Generics preferred | Double evaluation |
| Array iteration | Pointer arithmetic | Iterators | Memory safety |
| Include guards | Header protection | Module system | Automatic handling |

---

## Testing Macro Code

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_max_macro() {
        assert_eq!(max2!(5, 3), 5);
        assert_eq!(max2!(2.5, 3.7), 3.7);
        
        // Test single evaluation
        let mut x = 0;
        let mut y = 0;
        let inc_x = || { x += 1; x };
        let inc_y = || { y += 1; y };
        max2!(inc_x(), inc_y());
        assert_eq!(x, 1);  // Called only once
        assert_eq!(y, 1);  // Called only once
    }
}
```

---

## Key Takeaways

1. **Rust macros are not text substitution**
   - They work with syntax trees
   - Type checking after expansion
   - Hygienic by default

2. **Prefer functions over macros when possible**
   - Easier to debug
   - Better error messages
   - Type inference works better

3. **Safety improvements over C**
   - No double evaluation bugs
   - No variable capture
   - No precedence issues
   - Compile-time checking

4. **Module system superiority**
   - No include guards needed
   - Clear dependencies
   - Privacy control
   - No multiple inclusion

---
