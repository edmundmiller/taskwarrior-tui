// Simple backend test without TUI
use std::path::PathBuf;

// Include the backend modules directly
mod backend {
    pub use crate::backend::*;
}

// Mock the needed modules
mod config {
    pub struct Config {
        pub uda_backend: String,
        pub uda_taskchampion_data_dir: Option<String>,
        pub uda_taskchampion_server_config: Option<String>,
    }
    
    impl Config {
        pub fn new() -> Self {
            Self {
                uda_backend: "cli".to_string(),
                uda_taskchampion_data_dir: None,
                uda_taskchampion_server_config: None,
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Backend Integration Test ===");
    
    // Test CLI backend (like TUI would)
    let config = config::Config::new();
    
    println!("Config backend: {}", config.uda_backend);
    
    let backend_config = match config.uda_backend.as_str() {
        "taskchampion" => {
            println!("Would use TaskChampion backend");
            #[cfg(feature = "taskchampion-backend")]
            {
                backend::BackendConfig::TaskChampion {
                    data_dir: config.uda_taskchampion_data_dir.as_ref().map(|s| PathBuf::from(s)),
                    server_config: config.uda_taskchampion_server_config.clone(),
                }
            }
            #[cfg(not(feature = "taskchampion-backend"))]
            {
                backend::BackendConfig::Cli
            }
        }
        _ => {
            println!("Using CLI backend");
            backend::BackendConfig::Cli
        }
    };
    
    let backend = backend::create_backend(backend_config)?;
    println!("Backend created successfully");
    
    // Test task export (like TUI does on startup)
    println!("Testing task export...");
    let tasks = backend.export_tasks("", "next", "")?;
    
    println!("SUCCESS: Exported {} tasks", tasks.len());
    
    if tasks.is_empty() {
        println!("⚠️  No tasks found - this would cause blank TUI");
    } else {
        println!("✅ Tasks found - TUI should show them");
        println!("First 3 tasks:");
        for (i, task) in tasks.iter().take(3).enumerate() {
            println!("  {}. {} ({})", i + 1, task.description(), format!("{:?}", task.status()));
        }
    }
    
    Ok(())
}

// Include the backend modules inline since we can't use the full project structure
include!("src/backend/mod.rs");