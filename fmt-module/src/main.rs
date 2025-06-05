use std::fmt;

// Struct tùy chỉnh để demo Display trait
struct Point {
    x: i32,
    y: i32,
}

// Implement Display trait cho Point
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Implement Debug trait cho Point
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

fn main() {
    println!("=== Basic Formatting ===");
    // Display formatting
    println!("{}", 42); // Basic
    println!("{:?}", vec![1, 2, 3]); // Debug
    println!("{:#?}", vec![1, 2, 3]); // Pretty Debug

    println!("\n=== Number Formatting ===");
    // Integer formatting
    let number = 42;
    println!("Decimal: {}", 42);
    println!("Binary: {number:b}");
    println!("Octal: {:o}", 42);
    println!("Hexadecimal: {:x}", 42);
    println!("Pretty Hexadecimal: {:#x}", 42);
    println!("Upper hex: {:X}", 42);

    // Float formatting
    let pi = 3.14159;
    println!("Default float: {pi}");
    println!("Two decimals: {pi:.2}");
    println!("Exponential: {:e}", 3.14159);
    println!("Upper exponential: {:E}", 3.14159);

    println!("\n=== Alignment and Padding ===");
    // Width and alignment
    println!("Right align: {:>10}", "hello"); // Căn phải trong 10 ký tự
    println!("Left align: {:<10}", "hello"); // Căn trái trong 10 ký tự
    println!("Center: {:^10}", "hello"); // Căn giữa trong 10 ký tự

    // Padding with custom character
    println!("Pad zeros: {:0>5}", "123"); // Pad với số 0
    println!("Pad stars: {:*>5}", "123"); // Pad với dấu *

    println!("\n=== Named Arguments ===");
    // Named arguments
    println!("{name} là {age} tuổi", name = "Alice", age = 30);

    println!("\n=== Custom Types ===");
    // Custom type formatting
    let point = Point { x: 10, y: 20 };
    println!("Display: {}", point); // Sử dụng Display trait
    println!("Debug: {:?}", point); // Sử dụng Debug trait
    println!("Pretty debug: {:#?}", point); // Pretty debug

    println!("\n=== Error Handling ===");
    // Error handling trong formatting
    let result = fmt::write(&mut String::new(), format_args!("Hello, {}!", "world"));
    println!("Format result: {:?}", result);
}
