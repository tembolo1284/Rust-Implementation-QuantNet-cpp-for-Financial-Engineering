# Level 1: Basic C/C++ Features in Rust

## Overview
Level 1 introduces fundamental programming concepts, translating basic C/C++ features into Rust. This level emphasizes Rust's memory safety, explicit error handling, and modern syntax while covering essential programming constructs.

---

## Section 1.3: Console Input and Output

### Learning Objectives
- Basic I/O operations
- String manipulation
- User interaction

### Exercise 1: First Program
**C++ Focus**: `printf()` for output  
**Rust Translation**: `println!()` macro

```rust
println!("My first C-program");
println!("is a fact!");
println!("Good, isn't it?");
```

**Key Differences**:
- `printf()` → `println!()`
- No header files needed
- String literals are UTF-8 by default

### Exercise 2: User Input
**C++ Focus**: `scanf()` for input  
**Rust Translation**: `std::io::stdin().read_line()`

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read");
let number: i32 = input.trim().parse().expect("Invalid number");
```

**Key Concepts**:
- Explicit mutability with `mut`
- Error handling with `Result` type
- String parsing with type annotations

---

## Section 1.4: Control Structures

### Learning Objectives
- Conditional statements
- Loops and iteration
- Pattern matching

### Exercise 1: Number Classification
**C++ Focus**: `if-else` statements  
**Rust Translation**: `if` expressions and `match`

```rust
if number < 0 {
    println!("Negative");
} else if number > 0 {
    println!("Positive");
} else {
    println!("Zero");
}
```

### Exercise 2: Pattern Matching
**C++ Focus**: `switch` statement  
**Rust Translation**: `match` expression

```rust
match number {
    1..=12 => println!("Valid month"),
    _ => println!("Invalid month"),
}
```

**Advantages**:
- Exhaustive pattern matching
- No fall-through bugs
- Pattern guards and ranges

### Exercise 3: Loop Constructs
**C++ Focus**: `while`, `do-while`, `for` loops  
**Rust Translation**: `loop`, `while`, `for`

```rust
// Infinite loop with break
loop {
    if condition { break; }
}

// While loop
while count < 10 {
    count += 1;
}

// For loop with range
for i in 0..10 {
    println!("{}", i);
}
```

---

## Section 1.5: Functions

### Learning Objectives
- Function definitions
- Parameter passing
- Return values

### Exercise 1: Basic Functions
**C++ Focus**: Function declaration and definition  
**Rust Translation**: `fn` keyword with type annotations

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No return keyword needed for last expression
}
```

### Exercise 2: Multiple Parameters
**C++ Focus**: Pass by value vs reference  
**Rust Translation**: Ownership and borrowing

```rust
fn print_values(x: i32, y: &i32, s: &str) {
    println!("Owned: {}, Borrowed: {}, String slice: {}", x, y, s);
}
```

**Key Concepts**:
- Move semantics by default
- Explicit borrowing with `&`
- No null pointers

---

## Section 1.6: Macros (Preprocessor)

### Learning Objectives
- Macro definition and usage
- Code generation
- Conditional compilation

### Exercise 1: PRINT Macros
**C++ Focus**: `#define` macros  
**Rust Translation**: `macro_rules!`

```rust
macro_rules! print1 {
    ($x:expr) => {
        println!("{}", $x);
    };
}
```

### Exercise 2: MAX Macros
**C++ Focus**: Macro functions  
**Rust Translation**: Generic functions or macros

```rust
macro_rules! max2 {
    ($x:expr, $y:expr) => {
        if $x > $y { $x } else { $y }
    };
}
```

**Improvements**:
- Hygienic macros (no name collision)
- Pattern matching in macros
- Type safety after expansion

---

## Section 1.7: Pointers and Arrays

### Learning Objectives
- Pointer arithmetic (unsafe in Rust)
- Array manipulation
- Memory safety

### Exercise 1: Basic Pointers
**C++ Focus**: Raw pointers  
**Rust Translation**: References and smart pointers

```rust
let x = 5;
let r = &x;  // Reference (safe pointer)
println!("Value: {}", *r);  // Dereference
```

### Exercise 2: Swap Function
**C++ Focus**: Pointer parameters for swapping  
**Rust Translation**: Mutable references

```rust
fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}
```

### Exercise 3: Array Iteration
**C++ Focus**: Pointer arithmetic  
**Rust Translation**: Iterators

```rust
let arr = [1, 2, 3, 4, 5];

// Safe iteration
for &item in arr.iter() {
    println!("{}", item);
}

// Index access (bounds checked)
for i in 0..arr.len() {
    println!("{}", arr[i]);
}
```

### Exercise 4: Day Names Array
**C++ Focus**: Array of strings  
**Rust Translation**: Array of `&str` or `Vec<String>`

```rust
const DAYS: [&str; 7] = [
    "Sunday", "Monday", "Tuesday", "Wednesday",
    "Thursday", "Friday", "Saturday"
];
```

---

## Section 1.8: Structures

### Learning Objectives
- Struct definition
- Member functions
- Data encapsulation

### Exercise 1: Person Structure
**C++ Focus**: `struct` with public members  
**Rust Translation**: Struct with impl block

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    
    fn display(&self) {
        println!("{}, age {}", self.name, self.age);
    }
}
```

### Exercise 2: Article Structure
**C++ Focus**: Member functions  
**Rust Translation**: Methods and associated functions

```rust
impl Article {
    // Associated function (like static)
    fn default_price() -> f64 {
        10.0
    }
    
    // Method (takes self)
    fn total_cost(&self, quantity: u32) -> f64 {
        self.price * quantity as f64
    }
}
```

---

## Section 1.9: File Input/Output

### Learning Objectives
- File operations
- Error handling
- Text processing

### Exercise 1: Character Echo
**C++ Focus**: `getchar()` and `putchar()`  
**Rust Translation**: `stdin().bytes()`

```rust
use std::io::{self, Read};

for byte in io::stdin().bytes() {
    match byte {
        Ok(b) => print!("{}", b as char),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Exercise 2: File Writing
**C++ Focus**: `FILE*` and `fprintf()`  
**Rust Translation**: `File` and `write!()`

```rust
use std::fs::File;
use std::io::Write;

let mut file = File::create("output.txt")?;
writeln!(file, "Hello, file!")?;
```

**Advantages**:
- RAII file handling
- Explicit error handling with `Result`
- No manual `fclose()` needed

---

## Key Rust Advantages in Level 1

### Memory Safety
- No null pointers
- No buffer overflows
- No use-after-free
- Automatic memory management

### Error Handling
- `Result<T, E>` for recoverable errors
- `Option<T>` for optional values
- No silent failures
- Forced error handling

### Modern Features
- Pattern matching
- Type inference
- Iterators
- Closures
- Generics

### Zero-Cost Abstractions
- High-level features compile to efficient code
- No runtime overhead
- Performance equivalent to C

---

## Common Patterns and Translations

| C/C++ Pattern | Rust Equivalent | Benefit |
|---------------|-----------------|---------|
| `printf()` | `println!()` | Type-safe formatting |
| `scanf()` | `stdin().read_line()` | Error handling |
| `malloc/free` | `Box::new()` / automatic | Memory safety |
| `NULL` | `Option<T>` | Null safety |
| `switch` | `match` | Exhaustiveness |
| `#define` | `macro_rules!` | Hygienic macros |
| Raw pointers | References | Borrow checking |
| Manual loops | Iterators | Safer, cleaner |

---

## Compilation and Running

### Single Exercise
```bash
cd Level1/1.X/exerciseY
cargo build   # Compile
cargo run     # Run
cargo test    # Test
```

### With Input
```bash
cargo run < input.txt
echo "test data" | cargo run
```

### Debug vs Release
```bash
cargo run              # Debug build (default)
cargo run --release    # Optimized build
```

---

## Learning Path Notes

1. **Start with Section 1.3**: Basic I/O establishes foundation
2. **Practice error handling early**: Use `expect()` then migrate to `?`
3. **Understand ownership**: Critical for all Rust programming
4. **Use the compiler**: Rust's error messages are educational
5. **Write tests**: Built-in testing makes verification easy

---

## Common Beginner Mistakes

1. **Fighting the borrow checker**: Learn to work with it, not against it
2. **Overusing `clone()`**: Often borrowing is sufficient
3. **Ignoring compiler warnings**: They prevent real bugs
4. **Not handling errors**: Use `?` operator for propagation
5. **String confusion**: Understand `String` vs `&str`

---
