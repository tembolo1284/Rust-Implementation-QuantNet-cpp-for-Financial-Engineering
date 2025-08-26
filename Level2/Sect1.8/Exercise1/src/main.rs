// Exercise 1 - Section 1.8 Requirements:
// ---------------------------------------
// Write a C-program that prints the contents of a struct called Article.
// An Article has the following characteristics:
// * Article number
// * Quantity
// * Description (20 characters)
// The test program must create an Article of which the contents are
// assigned at initialization level.
// Printing the Article is done with a Print() function. This function
// gets the address of the structure as a parameter.
// Tip: Suppose that p is the pointer to the structure. It will allow
// the fields to be printed by (*p).fieldname or p->fieldname.
//
// Note: In Rust, we use references instead of pointers for safety

use std::fmt;

// Define the Article struct
#[derive(Debug)]
struct Article {
    article_number: u32,
    quantity: i32,
    description: String,  // In Rust, we use String instead of char array
}

// Implementation block for Article methods
impl Article {
    // Constructor function to create a new Article
    fn new(article_number: u32, quantity: i32, description: &str) -> Self {
        // Truncate description to 20 characters if needed
        let mut desc = description.to_string();
        if desc.len() > 20 {
            desc.truncate(20);
        }
        
        Article {
            article_number,
            quantity,
            description: desc,
        }
    }
}

// Print function that takes a reference to Article (like a pointer in C)
fn print(article: &Article) {
    println!("╔════════════════════════════════════════╗");
    println!("║           ARTICLE DETAILS              ║");
    println!("╚════════════════════════════════════════╝");
    println!("Article Number: {}", article.article_number);
    println!("Quantity:       {}", article.quantity);
    println!("Description:    {}", article.description);
}

// Alternative print function showing different access patterns
fn print_verbose(article: &Article) {
    println!("\n╔════════════════════════════════════════╗");
    println!("║      DETAILED ARTICLE INFORMATION      ║");
    println!("╚════════════════════════════════════════╝");
    
    // In C, we'd use p->field or (*p).field
    // In Rust, the dot operator works with references automatically
    println!("┌─────────────────┬──────────────────────┐");
    println!("│ Field           │ Value                │");
    println!("├─────────────────┼──────────────────────┤");
    println!("│ Article Number  │ {:20} │", article.article_number);
    println!("│ Quantity        │ {:20} │", article.quantity);
    println!("│ Description     │ {:20} │", article.description);
    println!("│ Desc. Length    │ {:20} │", article.description.len());
    println!("└─────────────────┴──────────────────────┘");
}

// Function demonstrating manual dereferencing (similar to C's (*p).field)
fn print_with_deref(article: &Article) {
    println!("\n╔════════════════════════════════════════╗");
    println!("║   MANUAL DEREFERENCE DEMONSTRATION     ║");
    println!("╚════════════════════════════════════════╝");
    
    // Explicitly dereference (like (*p).field in C)
    println!("Article Number: {}", (*article).article_number);
    println!("Quantity:       {}", (*article).quantity);
    println!("Description:    {}", (*article).description);
    
    println!("\nNote: In Rust, article.field and (*article).field are equivalent");
    println!("when 'article' is a reference. Rust auto-dereferences!");
}

// Custom Display implementation for pretty printing
impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Article #{}: {} units of \"{}\"", 
               self.article_number, self.quantity, self.description)
    }
}

fn main() {
    println!("Structure (Struct) Demonstration");
    println!("================================\n");
    
    // Create an Article with initialization
    // In C: struct Article art1 = {1001, 50, "Hammer"};
    let article1 = Article {
        article_number: 1001,
        quantity: 50,
        description: String::from("Hammer"),
    };
    
    // Call the print function with a reference (address)
    print(&article1);
    
    // Create another article using the constructor
    let article2 = Article::new(2002, 100, "Screwdriver Set");
    print(&article2);
    
    // Test with long description (will be truncated)
    let article3 = Article::new(3003, 25, "This is a very long description that exceeds twenty characters");
    print_verbose(&article3);
    
    // Demonstrate different print methods
    println!("\n╔════════════════════════════════════════╗");
    println!("║      DIFFERENT ACCESS PATTERNS         ║");
    println!("╚════════════════════════════════════════╝");
    
    let article4 = Article {
        article_number: 4004,
        quantity: 75,
        description: String::from("Nails (Box)"),
    };
    
    println!("\n1. Using automatic dereference:");
    print(&article4);
    
    println!("\n2. Using manual dereference:");
    print_with_deref(&article4);
    
    println!("\n3. Using Display trait:");
    println!("{}", article4);
    
    // Multiple articles in an array
    println!("\n╔════════════════════════════════════════╗");
    println!("║         ARRAY OF STRUCTURES            ║");
    println!("╚════════════════════════════════════════╝");
    
    let inventory = [
        Article::new(5001, 10, "Wrench"),
        Article::new(5002, 20, "Pliers"),
        Article::new(5003, 30, "Saw"),
    ];
    
    for (i, item) in inventory.iter().enumerate() {
        println!("\nItem {}:", i + 1);
        print(item);  // item is already a reference here
    }
    
    // Memory layout explanation
    println!("\n╔════════════════════════════════════════╗");
    println!("║         MEMORY LAYOUT                  ║");
    println!("╚════════════════════════════════════════╝");
    println!("C struct layout:");
    println!("  struct Article {{");
    println!("      int article_number;    // 4 bytes");
    println!("      int quantity;          // 4 bytes");
    println!("      char description[21];  // 21 bytes (20 + \\0)");
    println!("  }};  // Total: ~32 bytes with padding");
    println!();
    println!("Rust struct layout:");
    println!("  struct Article {{");
    println!("      article_number: u32,   // 4 bytes");
    println!("      quantity: i32,         // 4 bytes");
    println!("      description: String,   // 24 bytes (pointer+len+cap)");
    println!("  }}  // Total: 32 bytes typically");
    
    // Pointer notation comparison
    println!("\n╔════════════════════════════════════════╗");
    println!("║    C vs RUST POINTER/REFERENCE SYNTAX  ║");
    println!("╚════════════════════════════════════════╝");
    println!("┌──────────────┬─────────────────────────┐");
    println!("│ C Syntax     │ Rust Syntax             │");
    println!("├──────────────┼─────────────────────────┤");
    println!("│ Article *p   │ article: &Article       │");
    println!("│ &article     │ &article                │");
    println!("│ p->field     │ article.field           │");
    println!("│ (*p).field   │ (*article).field        │");
    println!("│              │ (or just article.field) │");
    println!("└──────────────┴─────────────────────────┘");
    
    // Show actual memory addresses
    println!("\n╔════════════════════════════════════════╗");
    println!("║      MEMORY ADDRESSES                  ║");
    println!("╚════════════════════════════════════════╝");
    println!("Address of article1: {:p}", &article1);
    println!("Address of article2: {:p}", &article2);
    println!("Size of Article: {} bytes", std::mem::size_of::<Article>());
}

