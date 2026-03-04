// Halt.rs Rust Example
// This example demonstrates how to use the Halt.rs circuit breaker
// and backpressure queue in a Rust application.

use halt::{CircuitBreaker, BackpressureQueue, AuditLogger, Priority};
use tokio;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Halt.rs Rust Example");

    // Initialize components
    let circuit_breaker = CircuitBreaker::new(5, 30); // 5 calls per 30 seconds
    let backpressure_queue = BackpressureQueue::new(1000);
    let audit_logger = AuditLogger::new();

    // Example 1: Circuit Breaker
    println!("\n=== Circuit Breaker Example ===");

    // Register some calls
    for i in 0..3 {
        let result = circuit_breaker.register_call("AgentA", "AgentB", i == 2); // Last call is terminal
        match result {
            Ok(_) => println!("Call {}: OK", i + 1),
            Err(err) => println!("Call {}: {}", i + 1, err),
        }
    }

    // Example 2: Backpressure Queue
    println!("\n=== Backpressure Queue Example ===");

    // Push tasks with different priorities
    let task1 = halt::Task {
        id: Uuid::new_v4(),
        name: "High Priority Reasoning".to_string(),
        priority: Priority::High,
        payload: r#"{"type": "reasoning", "model": "gpt-4", "prompt": "Analyze this code"}"#.to_string(),
    };

    let task2 = halt::Task {
        id: Uuid::new_v4(),
        name: "Low Priority Logging".to_string(),
        priority: Priority::Low,
        payload: r#"{"type": "logging", "message": "User action recorded"}"#.to_string(),
    };

    // Push tasks
    backpressure_queue.push(task1).await?;
    backpressure_queue.push(task2).await?;

    // Pop tasks (should get high priority first)
    while let Some(task) = backpressure_queue.pop().await {
        println!("Processing task: {} (priority: {:?})", task.name, task.priority);
    }

    // Example 3: Audit Logging
    println!("\n=== Audit Logging Example ===");

    // Log some interactions
    audit_logger.log_interaction(
        "AgentA",
        "AgentB",
        vec!["token_abc".to_string(), "token_def".to_string()]
    );

    audit_logger.log_interaction(
        "AgentB",
        "AgentC",
        vec!["token_ghi".to_string()]
    );

    // Get topology map
    let topology = audit_logger.get_map_json();
    println!("Current topology: {}", topology);

    println!("\nHalt.rs example completed successfully!");
    Ok(())
}
