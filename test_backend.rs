// Simple test to verify backend functionality
use std::path::PathBuf;

// Import the backend modules
mod backend;
use backend::{BackendConfig, create_backend};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing TaskChampion backend...");
    
    // Test CLI backend
    println!("1. Testing CLI backend...");
    let cli_backend = create_backend(BackendConfig::Cli)?;
    println!("   CLI backend created successfully");
    
    // Test TaskChampion backend
    #[cfg(feature = "taskchampion-backend")]
    {
        println!("2. Testing TaskChampion backend...");
        let tc_backend = create_backend(BackendConfig::TaskChampion {
            data_dir: Some(PathBuf::from("/tmp/test-tc")),
            server_config: None,
        })?;
        println!("   TaskChampion backend created successfully");
        
        // Test basic operations
        println!("3. Testing task export...");
        let tasks = tc_backend.export_tasks("", "next", "")?;
        println!("   Exported {} tasks", tasks.len());
        
        if tasks.is_empty() {
            println!("4. Testing task creation...");
            tc_backend.add_task("Test task from backend", &["test"])?;
            println!("   Test task created successfully");
            
            let tasks_after = tc_backend.export_tasks("", "next", "")?;
            println!("   Exported {} tasks after creation", tasks_after.len());
        }
    }
    
    #[cfg(not(feature = "taskchampion-backend"))]
    {
        println!("2. TaskChampion backend not available (feature disabled)");
    }
    
    println!("All tests completed successfully!");
    Ok(())
}