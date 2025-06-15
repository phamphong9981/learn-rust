use std::{cell::RefCell, rc::Rc};

fn main() {
    let a = String::from("Hello, world!");
    let rc_box = Rc::new(a);
    let rc_box2 = rc_box.clone();
    println!("rc_box: {}", rc_box);
    println!("rc_box2: {}", rc_box2);

    // mutable reference
    let shared_data = Rc::new(RefCell::new(String::from("Hello")));
    let data_clone = Rc::clone(&shared_data);

    shared_data.borrow_mut().push_str(", Rust!");

    println!("Data: {}", data_clone.borrow()); // In ra: Hello, Rust!
}
