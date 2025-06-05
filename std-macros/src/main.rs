mod dbg;
mod option_env;
use dotenv::dotenv;
use std::env;

fn main() {
    println!(
        "{:^120}",
        "============================= DBG ============================="
    );
    dbg::factorial(5);

    println!(
        "{:^120}",
        "============================= OPTION_ENV ============================="
    );
    println!(
        "Home env: {:?}",
        match option_env::get_home_env() {
            Some(home) => home,
            None => "HOME not set",
        }
    );
    println!(
        "Test env: {:?} because env from compile time => TEST=hello cargo run",
        match option_env::get_test_env() {
            Some(test) => test,
            None => "TEST not set",
        }
    );
    // print in center
    println!(
        "{:^120}",
        "============================= ENV ============================="
    );
    dotenv().ok();
    println!(
        "Home env: {:?}",
        env::var("HOME").unwrap_or("HOME not set".to_string())
    );
    println!(
        "Test env: {:?}",
        env::var("TEST").unwrap_or("TEST not set".to_string())
    );
}
