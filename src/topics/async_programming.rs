use tokio;

/// Demonstrates basic async/await functionality with multiple tasks
pub async fn basic_async_example() {
    println!("Starting async example...");

    // Create an async task that sleeps for a while
    let task1 = tokio::spawn(async {
        println!("Task 1: Starting");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("Task 1: Finished after 2 second");
    });

    let task2 = tokio::spawn(async {
        println!("Task 2: Starting");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("Task 2: Finished after 1 seconds");
    });

    // Wait for both tasks to complete
    let _ = tokio::join!(task1, task2);

    println!("All tasks completed!");
}
