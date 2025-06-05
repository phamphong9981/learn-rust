mod topics;
use my_macros::{HelloWorld, log_function, make_greeting};

#[derive(HelloWorld)]
struct Person {
    name: String,
}

#[log_function]
fn test_function() {
    println!("Đây là nội dung của hàm");
}

#[tokio::main]
async fn main() {
    // Sử dụng procedural macro
    make_greeting!("Phong");

    // Sử dụng derive macro
    let person = Person {
        name: "Phong".to_string(),
    };
    person.hello_world();

    // Sử dụng attribute macro
    test_function();

    println!("=== Running Async Programming Examples ===");
    topics::async_programming::basic_async_example().await;
}
