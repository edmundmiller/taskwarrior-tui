// Test the backend functionality directly
use taskwarrior_tui::backend::{BackendConfig, create_backend, TaskBackend};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Direct Backend Test ===");
    
    // Test CLI backend (which should be working now)
    println!("1. Creating CLI backend...");
    let backend = create_backend(BackendConfig::Cli)?;
    println!("   ✅ CLI backend created");
    
    // Test export with "next" report (simulating TUI)
    println!("2. Exporting tasks with 'next' report...");
    let tasks = backend.export_tasks("", "next", "")?;
    println!("   ✅ Exported {} tasks", tasks.len());
    
    if tasks.is_empty() {
        println!("   ⚠️  WARNING: No tasks returned");
        println!("   This would cause the TUI to show a blank page.");
        
        // Try with "all" report to see if there are any tasks at all
        println!("3. Trying 'all' report...");
        let all_tasks = backend.export_tasks("", "all", "")?;
        println!("   📋 'all' report returned {} tasks", all_tasks.len());
        
        if all_tasks.is_empty() {
            println!("   No tasks in database at all.");
        } else {
            println!("   Tasks exist but not showing in 'next' report.");
            println!("   Check your taskwarrior 'next' report configuration.");
        }
    } else {
        println!("   ✅ SUCCESS: Found {} tasks for 'next' report", tasks.len());
        println!("   📋 First 5 tasks:");
        for (i, task) in tasks.iter().take(5).enumerate() {
            println!("     {}. {} ({})", 
                    i + 1, 
                    task.description(), 
                    format!("{:?}", task.status()));
        }
        println!("   🎉 The TUI should now show these tasks!");
    }
    
    Ok(())
}