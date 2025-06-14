mod topics;
use decl_macros::{calculate, create_function, log_vars, vec_of_strings};
use my_macros::{log_function, make_greeting, make_greeting2, HelloWorld};

#[derive(HelloWorld)]
struct Person {
    name: String,
}

#[log_function]
fn test_function() {
    println!("Đây là nội dung của hàm");
}

// Sử dụng macro create_function để tạo hàm mới
create_function!(say_hi, println!("Xin chào từ hàm được tạo bởi macro!"));

#[tokio::main]
async fn main() {
    // Sử dụng function macro
    println!("=== Running Function Macro ===");
    make_greeting!("Phong");
    make_greeting2!("Phong", "25");

    // Sử dụng derive macro
    println!("=== Running Derive Macro ===");
    let person = Person {
        name: "Phong".to_string(),
    };
    person.hello_world();

    // Sử dụng attribute macro
    println!("=== Running Attribute Macro ===");
    test_function();

    // Sử dụng vec_of_strings macro
    println!("============== Running Declarative Macros ==============");
    let strings = vec_of_strings!("Hello", "World", "from", "macro");
    println!("Vector of strings: {:?}", strings);

    // Sử dụng calculate macro
    let sum = calculate!(+ 1, 2, 3, 4, 5);
    let product = calculate!(*1, 2, 3, 4, 5);
    println!("Sum: {}, Product: {}", sum, product);

    // Sử dụng hàm được tạo bởi macro
    say_hi();

    // Sử dụng log_vars macro
    let x = 42;
    let y = "Hello";
    let z = vec![1, 2, 3];
    log_vars!(x, y, z);

    println!("=== Running Async Programming Examples ===");
    topics::async_programming::basic_async_example().await;

    println!("=== Running Struct Examples ===");
    let user = topics::structs::User::new("Phong".to_string(), 25);
    println!("User: {}", user.get_age());
    println!("User: {}", user.get_name());
}
