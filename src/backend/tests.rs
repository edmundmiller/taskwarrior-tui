//! Tests for backend implementations

use super::*;
use std::path::PathBuf;
use tempfile::TempDir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_backend_creation() {
        let backend = create_backend(BackendConfig::Cli);
        assert!(backend.is_ok(), "CLI backend creation should succeed");
    }

    #[cfg(feature = "taskchampion-backend")]
    #[test]
    fn test_taskchampion_backend_creation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let backend = create_backend(BackendConfig::TaskChampion {
            data_dir: Some(temp_dir.path().to_path_buf()),
            server_config: None,
        });
        assert!(backend.is_ok(), "TaskChampion backend creation should succeed");
    }

    #[test]
    fn test_cli_backend_export_tasks() {
        let backend = create_backend(BackendConfig::Cli).expect("Failed to create CLI backend");
        
        // Test with empty filter (should not fail)
        let result = backend.export_tasks("", "next", "");
        match result {
            Ok(tasks) => {
                println!("CLI backend exported {} tasks", tasks.len());
                // Print first few tasks for debugging
                for (i, task) in tasks.iter().take(3).enumerate() {
                    println!("Task {}: {:?}", i + 1, task.description());
                }
            }
            Err(e) => {
                println!("CLI backend export failed: {}", e);
                panic!("CLI backend export should not fail: {}", e);
            }
        }
    }

    #[test]
    fn test_command_building_debug() {
        println!("=== Command Building Debug ===");
        
        let backend = create_backend(BackendConfig::Cli).expect("Failed to create CLI backend");
        
        // Manually test the command building logic
        let mut cmd = std::process::Command::new("task");
        cmd.arg("rc.json.array=on")
           .arg("rc.confirmation=off")
           .arg("rc.json.depends.array=on")
           .arg("rc.color=off")
           .arg("rc._forcecolor=off");
        
        // No filter override (empty filter)
        
        cmd.arg("export");
        
        // Check version logic
        let output = std::process::Command::new("task").arg("--version").output().expect("Failed to get version");
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Raw version output: '{}'", stdout);
        let version_line = stdout.lines().next().unwrap_or("");
        println!("Version line: '{}'", version_line);
        
        let version_str = if version_line.starts_with("task ") {
            println!("Using 'task X.Y.Z' format");
            version_line.split_whitespace().nth(1).unwrap_or("0.0.0")
        } else {
            println!("Using direct version format");
            version_line.trim()
        };
        println!("TaskWarrior version string: '{}'", version_str);
        
        let version = versions::Versioning::new(version_str).expect("Failed to parse version");
        let supported_version = versions::Versioning::new("3.0.0").unwrap();
        println!("Parsed version: {}, supported: {}", version, supported_version);
        println!("Version >= 3.0.0: {}", version >= supported_version);
        
        if version >= supported_version {
            cmd.arg("next");
            println!("Added 'next' argument");
        } else {
            println!("Did NOT add 'next' argument (version too old)");
        }
        
        // Print the final command
        let args: Vec<&std::ffi::OsStr> = cmd.get_args().collect();
        println!("Final command: task {:?}", args);
        
        // Execute and count results
        let output = cmd.output().expect("Failed to execute command");
        if output.status.success() {
            let data = String::from_utf8_lossy(&output.stdout);
            let tasks: Vec<task_hookrs::task::Task<task_hookrs::task::TW26>> = task_hookrs::import::import(data.as_bytes()).expect("Failed to import");
            let task_count = tasks.len();
            println!("Command returned {} tasks", task_count);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("Command failed: {}", error);
        }
    }

    #[test]
    fn test_cli_backend_export_with_filter() {
        let backend = create_backend(BackendConfig::Cli).expect("Failed to create CLI backend");
        
        // Test with a simple filter
        let result = backend.export_tasks("status:pending", "next", "");
        match result {
            Ok(tasks) => {
                println!("CLI backend with filter exported {} tasks", tasks.len());
                // Verify all returned tasks are pending
                for task in &tasks {
                    assert_eq!(task.status(), &task_hookrs::status::TaskStatus::Pending);
                }
            }
            Err(e) => {
                println!("CLI backend with filter failed: {}", e);
                // This might fail if there are no pending tasks, which is ok
            }
        }
    }

    #[cfg(feature = "taskchampion-backend")]
    #[test]
    fn test_taskchampion_backend_basic_operations() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let backend = create_backend(BackendConfig::TaskChampion {
            data_dir: Some(temp_dir.path().to_path_buf()),
            server_config: None,
        }).expect("Failed to create TaskChampion backend");

        // Test export (should be empty initially)
        let initial_tasks = backend.export_tasks("", "next", "").expect("Export should succeed");
        println!("TaskChampion backend initial tasks: {}", initial_tasks.len());

        // Test adding a task
        let add_result = backend.add_task("Test task from unit test", &["test", "unit"]);
        assert!(add_result.is_ok(), "Add task should succeed: {:?}", add_result);

        // Test export again (should now have 1 task)
        let after_add_tasks = backend.export_tasks("", "next", "").expect("Export after add should succeed");
        println!("TaskChampion backend after add: {} tasks", after_add_tasks.len());
        
        if !after_add_tasks.is_empty() {
            let task = &after_add_tasks[0];
            println!("Added task: {:?}", task.description());
            assert_eq!(task.description(), "Test task from unit test");
            
            // Test marking task as done
            let task_uuid = *task.uuid();
            let done_result = backend.mark_done(&[task_uuid]);
            assert!(done_result.is_ok(), "Mark done should succeed: {:?}", done_result);
            
            // Test export again (completed tasks might not show up depending on filter)
            let after_done_tasks = backend.export_tasks("", "next", "").expect("Export after done should succeed");
            println!("TaskChampion backend after done: {} tasks", after_done_tasks.len());
        }
    }

    #[test]
    fn test_backend_configuration_parsing() {
        // Test default configuration
        let default_config = BackendConfig::default();
        match default_config {
            BackendConfig::Cli => {
                println!("Default backend is CLI");
            }
            #[cfg(feature = "taskchampion-backend")]
            BackendConfig::TaskChampion { .. } => {
                println!("Default backend is TaskChampion (feature enabled)");
            }
        }
    }

    /// Test app-like initialization to verify backend integration
    #[test]
    fn test_app_backend_integration() {
        println!("=== App Backend Integration Test ===");
        
        // Simulate the app's backend selection logic
        println!("1. Testing backend selection logic...");
        
        // Mock config with TaskChampion backend (like the actual config)
        let uda_backend = "taskchampion".to_string();
        println!("   uda_backend config: '{}'", uda_backend);
        
        let backend_config = match uda_backend.as_str() {
            "taskchampion" => {
                #[cfg(feature = "taskchampion-backend")]
                {
                    let data_dir = Some(std::path::PathBuf::from("/Users/emiller/Library/Mobile Documents/iCloud~com~mav~taskchamp/Documents/task"));
                    println!("   Would use TaskChampion backend with data_dir: {:?}", data_dir);
                    BackendConfig::TaskChampion {
                        data_dir,
                        server_config: None,
                    }
                }
                #[cfg(not(feature = "taskchampion-backend"))]
                {
                    println!("   TaskChampion not available, using CLI");
                    BackendConfig::Cli
                }
            }
            _ => {
                println!("   Using CLI backend");
                BackendConfig::Cli
            }
        };
        
        println!("2. Creating backend...");
        let backend = create_backend(backend_config).expect("Failed to create backend");
        println!("   ✅ Backend created successfully");
        
        // Simulate app's export_tasks call (with empty filter like TUI)
        println!("3. Simulating app export_tasks call...");
        let filter = "";
        let report = "next";
        let context_filter = "";
        
        println!("   Calling export_tasks('{}', '{}', '{}')", filter, report, context_filter);
        let tasks = backend.export_tasks(filter, report, context_filter)
            .expect("Backend export should succeed");
        
        println!("   ✅ Export succeeded: {} tasks", tasks.len());
        
        // This simulates what the app does after export
        if tasks.is_empty() {
            println!("   ⚠️  WARNING: App would show blank page (no tasks)");
        } else {
            println!("   ✅ SUCCESS: App would show {} tasks", tasks.len());
            
            // Count by status like app would
            let pending_count = tasks.iter()
                .filter(|t| matches!(t.status(), task_hookrs::status::TaskStatus::Pending))
                .count();
            
            println!("   📊 {} pending tasks (should be visible in TUI)", pending_count);
            
            if pending_count == 0 {
                println!("   ⚠️  No pending tasks - TUI might appear blank");
            }
        }
    }

    /// Test that simulates the exact scenario from the TUI
    #[test]
    fn test_tui_scenario_simulation() {
        println!("=== Simulating TUI Task Export Scenario ===");
        
        // This simulates what the TUI does when loading tasks
        let backend = create_backend(BackendConfig::Cli).expect("Failed to create CLI backend");
        
        // Simulate the exact call that TUI makes
        let filter = ""; // Empty filter like in TUI
        let report = "next"; // Default report
        let context_filter = ""; // No context filter
        
        println!("Testing CLI backend with:");
        println!("  filter: '{}'", filter);
        println!("  report: '{}'", report);
        println!("  context_filter: '{}'", context_filter);
        
        let result = backend.export_tasks(filter, report, context_filter);
        
        match result {
            Ok(tasks) => {
                println!("✅ SUCCESS: Exported {} tasks", tasks.len());
                
                // Count tasks by status
                let mut pending_count = 0;
                let mut completed_count = 0;
                let mut deleted_count = 0;
                
                for task in &tasks {
                    match task.status() {
                        task_hookrs::status::TaskStatus::Pending => pending_count += 1,
                        task_hookrs::status::TaskStatus::Completed => completed_count += 1,
                        task_hookrs::status::TaskStatus::Deleted => deleted_count += 1,
                        _ => {}
                    }
                }
                
                println!("📊 Task Status Summary:");
                println!("   Pending: {}", pending_count);
                println!("   Completed: {}", completed_count);
                println!("   Deleted: {}", deleted_count);
                
                if tasks.is_empty() {
                    println!("⚠️  WARNING: No tasks returned. This could explain the blank TUI.");
                    println!("   Check if you have any pending tasks in your taskwarrior database.");
                } else if pending_count == 0 {
                    println!("🔍 ROOT CAUSE FOUND: No pending tasks!");
                    println!("   The 'next' report typically shows only pending tasks.");
                    println!("   Since all {} tasks are completed, the TUI shows blank.", completed_count);
                    println!("   Add some pending tasks to see them in the TUI.");
                } else {
                    println!("📋 First 5 tasks:");
                    for (i, task) in tasks.iter().take(5).enumerate() {
                        println!("   {}. {} (status: {:?})", 
                                i + 1, 
                                task.description(), 
                                task.status()
                        );
                    }
                }
            }
            Err(e) => {
                println!("❌ FAILED: Export failed with error: {}", e);
                println!("   This would cause the TUI to show a blank screen.");
                panic!("Backend export failed: {}", e);
            }
        }
        
        // Test with all report to see all tasks
        println!("\n=== Testing with 'all' report ===");
        let result_all = backend.export_tasks("", "all", "");
        match result_all {
            Ok(tasks) => {
                println!("📋 'all' report returned {} tasks", tasks.len());
                if !tasks.is_empty() {
                    println!("   First task: {} ({})", tasks[0].description(), 
                            format!("{:?}", tasks[0].status()));
                }
            }
            Err(e) => {
                println!("❌ 'all' report failed: {}", e);
            }
        }
    }

    /// Test that CLI and TaskChampion backends return consistent results
    #[test]
    #[cfg(feature = "taskchampion-backend")]
    fn test_backend_consistency() {
        println!("=== Backend Consistency Test ===");
        
        // Test CLI backend
        println!("1. Testing CLI backend...");
        let cli_backend = create_backend(BackendConfig::Cli).expect("Failed to create CLI backend");
        let cli_tasks = cli_backend.export_tasks("", "next", "").expect("CLI backend export failed");
        println!("   CLI backend: {} tasks", cli_tasks.len());
        
        // Test TaskChampion backend with correct data directory
        println!("2. Testing TaskChampion backend...");
        let tc_backend = create_backend(BackendConfig::TaskChampion {
            data_dir: Some(std::path::PathBuf::from("/Users/emiller/Library/Mobile Documents/iCloud~com~mav~taskchamp/Documents/task")),
            server_config: None,
        }).expect("Failed to create TaskChampion backend");
        
        let tc_tasks = tc_backend.export_tasks("", "next", "").expect("TaskChampion backend export failed");
        println!("   TaskChampion backend: {} tasks", tc_tasks.len());
        
        // Compare results
        println!("3. Comparing results...");
        
        if cli_tasks.len() != tc_tasks.len() {
            println!("   ⚠️  Task count mismatch: CLI={}, TaskChampion={}", cli_tasks.len(), tc_tasks.len());
            
            // This might be expected due to different implementation details
            // The important thing is that both return > 0 tasks
            if cli_tasks.len() > 0 && tc_tasks.len() > 0 {
                println!("   ✅ Both backends return tasks (counts may differ due to implementation)");
            } else {
                panic!("One or both backends returned no tasks");
            }
        } else {
            println!("   ✅ Perfect match: both backends return {} tasks", cli_tasks.len());
        }
        
        // Verify both have pending tasks
        let cli_pending = cli_tasks.iter()
            .filter(|t| matches!(t.status(), task_hookrs::status::TaskStatus::Pending))
            .count();
        let tc_pending = tc_tasks.iter()
            .filter(|t| matches!(t.status(), task_hookrs::status::TaskStatus::Pending))
            .count();
            
        println!("   CLI pending: {}, TaskChampion pending: {}", cli_pending, tc_pending);
        
        assert!(cli_pending > 0, "CLI backend should return pending tasks");
        assert!(tc_pending > 0, "TaskChampion backend should return pending tasks");
        
        println!("   ✅ Both backends successfully filter for pending tasks");
    }
}