// Debug script to check backend behavior
use std::env;

// Set up logging
fn setup_logging() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();
}

mod backend;
use backend::{BackendConfig, create_backend, TaskBackend};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();
    
    println!("=== Backend Debug Test ===");
    
    // Create CLI backend
    let backend = create_backend(BackendConfig::Cli)?;
    println!("Backend created successfully");
    
    // Test export with empty filter (simulating TUI behavior)
    println!("Calling export_tasks with empty filter and 'next' report...");
    let tasks = backend.export_tasks("", "next", "")?;
    
    println!("Exported {} tasks", tasks.len());
    
    // Show status breakdown
    let mut pending = 0;
    let mut completed = 0;
    
    for task in &tasks {
        match task.status() {
            task_hookrs::status::TaskStatus::Pending => pending += 1,
            task_hookrs::status::TaskStatus::Completed => completed += 1,
            _ => {}
        }
    }
    
    println!("Status breakdown: {} pending, {} completed", pending, completed);
    
    if !tasks.is_empty() {
        println!("First task: {} ({})", 
                tasks[0].description(), 
                format!("{:?}", tasks[0].status()));
    }
    
    Ok(())
}