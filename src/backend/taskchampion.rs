use anyhow::Result;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;
use task_hookrs::task::{Task, TaskBuilder};
use task_hookrs::status::TaskStatus;
use uuid::Uuid;

use super::TaskBackend;

/// TaskChampion-based backend for direct database access
pub struct TaskChampionBackend {
    replica: Mutex<taskchampion::Replica>,
}

impl TaskChampionBackend {
    pub fn new(data_dir: Option<PathBuf>, _server_config: Option<String>) -> Result<Self> {
        let storage_dir = data_dir.unwrap_or_else(|| {
            dirs::data_dir()
                .unwrap_or_else(|| std::env::current_dir().unwrap())
                .join("taskwarrior-tui")
                .join("taskchampion")
        });

        // Ensure the directory exists
        std::fs::create_dir_all(&storage_dir)?;

        let storage = taskchampion::StorageConfig::OnDisk {
            taskdb_dir: storage_dir,
        }
        .into_storage()?;

        let replica = taskchampion::Replica::new(storage);

        Ok(Self {
            replica: Mutex::new(replica),
        })
    }
}

impl TaskBackend for TaskChampionBackend {
    fn export_tasks(&self, filter: &str, report: &str, _context_filter: &str) -> Result<Vec<Task>> {
        let mut replica = self.replica.lock().unwrap();
        
        // Get all tasks from TaskChampion
        let tc_tasks = replica.all_tasks()?;
        let total_tasks = tc_tasks.len();
        
        let mut converted_tasks = Vec::new();
        
        for (tc_uuid, tc_task) in tc_tasks {
            // Convert TaskChampion task to task-hookrs format
            if let Some(task) = convert_taskchampion_to_hookrs(tc_uuid, tc_task)? {
                converted_tasks.push(task);
            }
        }
        
        // Determine the actual filter to apply
        let effective_filter = if filter.is_empty() {
            // Get the default filter for this report
            get_report_filter(report).unwrap_or_else(|| {
                log::warn!("Could not get filter for report '{}', using no filter", report);
                String::new()
            })
        } else {
            filter.to_string()
        };
        
        log::debug!("TaskChampion backend: Using filter: '{}'", effective_filter);
        
        // Apply filtering
        let mut filtered_tasks = if effective_filter.is_empty() {
            converted_tasks
        } else {
            converted_tasks.into_iter()
                .filter(|task| task_matches_taskwarrior_filter(task, &effective_filter))
                .collect()
        };
        
        // Apply limit if specified in filter
        if let Some(limit) = extract_limit_from_filter(&effective_filter) {
            filtered_tasks.truncate(limit);
            log::debug!("Applied limit of {} tasks", limit);
        }
        
        log::info!("TaskChampion backend: Exported {} tasks (filtered from {} total)", 
                  filtered_tasks.len(), total_tasks);
        Ok(filtered_tasks)
    }

    fn add_task(&self, description: &str, args: &[&str]) -> Result<()> {
        let mut replica = self.replica.lock().unwrap();
        
        // Create a new task with TaskChampion
        let new_task = replica.new_task(taskchampion::Status::Pending, description.to_string())?;
        let mut task_mut = new_task.into_mut(&mut replica);
        
        // Parse and apply additional arguments
        for arg in args {
            if let Some((key, value)) = parse_task_arg(arg) {
                match key.as_str() {
                    "project" => {
                        // TODO: TaskChampion doesn't seem to have project support in 0.4.1
                        // We'll implement this when we find the right API
                        log::debug!("Project setting not yet implemented: {}", value);
                    }
                    "priority" => {
                        // TODO: TaskChampion doesn't seem to have priority support in 0.4.1
                        // We'll implement this when we find the right API
                        log::debug!("Priority setting not yet implemented: {}", value);
                    }
                    _ => {
                        // Treat as tag
                        let tag = taskchampion::Tag::from_str(&key)?;
                        task_mut.add_tag(&tag)?;
                    }
                }
            }
        }
        
        log::info!("TaskChampion backend: Added task '{}'", description);
        Ok(())
    }

    fn mark_done(&self, task_uuids: &[Uuid]) -> Result<()> {
        let mut replica = self.replica.lock().unwrap();
        
        for &task_uuid in task_uuids {
            // Convert UUID format
            let tc_uuid = taskchampion::Uuid::from_bytes(task_uuid.as_bytes().clone());
            
            // Get the task and mark it as done
            if let Some(task) = replica.get_task(tc_uuid)? {
                let mut task_mut = task.into_mut(&mut replica);
                task_mut.done()?;
            }
        }
        
        log::info!("TaskChampion backend: Marked {} tasks as done", task_uuids.len());
        Ok(())
    }

    fn delete_tasks(&self, task_uuids: &[Uuid]) -> Result<()> {
        let mut replica = self.replica.lock().unwrap();
        
        for &task_uuid in task_uuids {
            // Convert UUID format
            let tc_uuid = taskchampion::Uuid::from_bytes(task_uuid.as_bytes().clone());
            
            // Get the task and delete it
            if let Some(task) = replica.get_task(tc_uuid)? {
                let mut task_mut = task.into_mut(&mut replica);
                task_mut.set_status(taskchampion::Status::Deleted)?;
            }
        }
        
        log::info!("TaskChampion backend: Deleted {} tasks", task_uuids.len());
        Ok(())
    }

    fn modify_tasks(&self, task_uuids: &[Uuid], modifications: &str) -> Result<()> {
        let mut replica = self.replica.lock().unwrap();
        
        for &task_uuid in task_uuids {
            // Convert UUID format
            let tc_uuid = taskchampion::Uuid::from_bytes(task_uuid.as_bytes().clone());
            
            // Get the task and modify it
            if let Some(task) = replica.get_task(tc_uuid)? {
                let mut task_mut = task.into_mut(&mut replica);
                apply_modifications(&mut task_mut, modifications)?;
            }
        }
        
        log::info!("TaskChampion backend: Modified {} tasks with '{}'", task_uuids.len(), modifications);
        Ok(())
    }

    fn get_task_details(&self, task_uuid: Uuid) -> Result<Option<String>> {
        let mut replica = self.replica.lock().unwrap();
        
        // Convert UUID format
        let tc_uuid = taskchampion::Uuid::from_bytes(task_uuid.as_bytes().clone());
        
        if let Some(task) = replica.get_task(tc_uuid)? {
            // Convert task to task-hookrs format and then to JSON
            if let Some(hookrs_task) = convert_taskchampion_to_hookrs(tc_uuid, task)? {
                let json = serde_json::to_string_pretty(&hookrs_task)?;
                log::info!("TaskChampion backend: Got details for task {}", task_uuid);
                return Ok(Some(json));
            }
        }
        
        log::info!("TaskChampion backend: No details found for task {}", task_uuid);
        Ok(None)
    }

    fn sync(&self) -> Result<()> {
        // Placeholder implementation
        // TODO: Implement actual synchronization with TaskChampion
        log::info!("TaskChampion backend: Syncing tasks");
        Ok(())
    }
}

/// Convert a TaskChampion task to task-hookrs format
fn convert_taskchampion_to_hookrs(
    tc_uuid: taskchampion::Uuid,
    tc_task: taskchampion::Task,
) -> Result<Option<Task>> {
    // Skip deleted tasks
    if tc_task.get_status() == taskchampion::Status::Deleted {
        return Ok(None);
    }
    
    // Convert UUID - TaskChampion uses uuid::Uuid, task-hookrs uses uuid::Uuid
    let uuid = Uuid::from_bytes(tc_uuid.as_bytes().clone());
    
    // Convert status
    let status = match tc_task.get_status() {
        taskchampion::Status::Pending => TaskStatus::Pending,
        taskchampion::Status::Completed => TaskStatus::Completed,
        taskchampion::Status::Deleted => return Ok(None), // Skip deleted tasks
    };
    
    // Build the task using TaskBuilder
    let mut builder = TaskBuilder::default();
    builder
        .uuid(uuid)
        .status(status)
        .description(tc_task.get_description().to_string());
    
    // Add tags if present
    let tags: Vec<String> = tc_task.get_tags().map(|tag| tag.to_string()).collect();
    if !tags.is_empty() {
        // TODO: task-hookrs doesn't seem to have a simple way to set tags via builder
        // We'll need to set them after building or find another approach
    }
    
    // Build the task
    let task = builder.build().map_err(|e| anyhow::anyhow!("Failed to build task: {}", e))?;
    
    Ok(Some(task))
}

/// Get the filter for a specific TaskWarrior report
fn get_report_filter(report: &str) -> Option<String> {
    // Execute `task show report.{report}.filter` to get the filter
    let output = std::process::Command::new("task")
        .arg("show")
        .arg(format!("report.{}.filter", report))
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse the output to extract the filter value
    // Format is typically:
    // Config Variable    Value
    // ------------------ ----------------------------------
    // report.next.filter status:pending -WAITING limit:page
    let mut in_data_section = false;
    
    for line in stdout.lines() {
        let trimmed = line.trim();
        
        // Skip until we find the data section (after the header)
        if trimmed.starts_with("---") {
            in_data_section = true;
            continue;
        }
        
        if in_data_section && trimmed.contains(&format!("report.{}.filter", report)) {
            // Split by whitespace and take everything after the variable name
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                // Join all parts after the first (variable name) to get the full filter
                let filter = parts[1..].join(" ");
                if !filter.is_empty() {
                    return Some(filter);
                }
            }
        }
    }
    
    None
}

/// Parse and apply TaskWarrior filter syntax
fn task_matches_taskwarrior_filter(task: &Task, filter: &str) -> bool {
    if filter.is_empty() {
        return true;
    }
    
    log::debug!("Applying filter '{}' to task: {}", filter, task.description());
    
    // Split filter into individual terms
    let terms: Vec<&str> = filter.split_whitespace().collect();
    
    for term in terms {
        if term.starts_with("status:") {
            let status_filter = &term[7..]; // Remove "status:"
            let task_status = format!("{:?}", task.status()).to_lowercase();
            if !task_status.contains(&status_filter.to_lowercase()) {
                log::debug!("Task status '{}' doesn't match filter '{}'", task_status, status_filter);
                return false;
            }
        } else if term.starts_with("-") && term.len() > 1 {
            // Negative filter (exclude)
            let exclude_term = &term[1..];
            if exclude_term.to_uppercase() == "WAITING" {
                // Check if task is waiting (has wait date)
                if task.wait().is_some() {
                    log::debug!("Task is waiting, excluded by '-WAITING' filter");
                    return false;
                }
            }
            // TODO: Add more negative filters
        } else if term.starts_with("project:") {
            let project_filter = &term[8..]; // Remove "project:"
            if let Some(task_project) = task.project() {
                if !task_project.to_lowercase().contains(&project_filter.to_lowercase()) {
                    log::debug!("Task project '{}' doesn't match filter '{}'", task_project, project_filter);
                    return false;
                }
            } else {
                // Task has no project but filter requires one
                log::debug!("Task has no project but filter requires '{}'", project_filter);
                return false;
            }
        } else if term.starts_with("limit:") {
            // Limit filters are handled at a higher level, ignore here
            continue;
        } else if term.starts_with("+") {
            // Tag filter
            let tag_filter = &term[1..];
            if let Some(tags) = task.tags() {
                let has_tag = tags.iter().any(|tag| tag.to_lowercase() == tag_filter.to_lowercase());
                if !has_tag {
                    log::debug!("Task doesn't have required tag '{}'", tag_filter);
                    return false;
                }
            } else {
                log::debug!("Task has no tags but filter requires '{}'", tag_filter);
                return false;
            }
        }
        // TODO: Add more filter types as needed
    }
    
    true
}

/// Extract limit value from filter (e.g., "limit:page" or "limit:10")
fn extract_limit_from_filter(filter: &str) -> Option<usize> {
    let terms: Vec<&str> = filter.split_whitespace().collect();
    
    for term in terms {
        if term.starts_with("limit:") {
            let limit_value = &term[6..]; // Remove "limit:"
            
            // Handle special values
            if limit_value == "page" {
                // TaskWarrior's default page size is typically 25
                return Some(25);
            } else if let Ok(num) = limit_value.parse::<usize>() {
                return Some(num);
            }
        }
    }
    
    None
}

/// Parse a task argument in the format "key:value" or just "key" for tags
fn parse_task_arg(arg: &str) -> Option<(String, String)> {
    if let Some(pos) = arg.find(':') {
        let key = arg[..pos].to_string();
        let value = arg[pos + 1..].to_string();
        Some((key, value))
    } else {
        // Treat as tag
        Some((arg.to_string(), String::new()))
    }
}

/// Apply modifications to a TaskChampion task
fn apply_modifications(task: &mut taskchampion::TaskMut, modifications: &str) -> Result<()> {
    let args: Vec<&str> = modifications.split_whitespace().collect();
    
    for arg in args {
        if let Some((key, value)) = parse_task_arg(arg) {
            match key.as_str() {
                "description" => {
                    task.set_description(value)?;
                }
                "project" => {
                    // TODO: TaskChampion doesn't seem to have project support in 0.4.1
                    log::debug!("Project modification not yet implemented: {}", value);
                }
                "priority" => {
                    // TODO: TaskChampion doesn't seem to have priority support in 0.4.1
                    log::debug!("Priority modification not yet implemented: {}", value);
                }
                _ => {
                    // Treat as tag
                    if value.is_empty() {
                        let tag = taskchampion::Tag::from_str(&key)?;
                        task.add_tag(&tag)?;
                    } else {
                        // For now, just add as tag since we don't have UDA support
                        let tag = taskchampion::Tag::from_str(&format!("{}:{}", key, value))?;
                        task.add_tag(&tag)?;
                    }
                }
            }
        }
    }
    
    Ok(())
}

