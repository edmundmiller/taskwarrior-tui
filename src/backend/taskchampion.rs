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
    fn export_tasks(&self, filter: &str, _report: &str, _context_filter: &str) -> Result<Vec<Task>> {
        let mut replica = self.replica.lock().unwrap();
        
        // Get all tasks from TaskChampion
        let tc_tasks = replica.all_tasks()?;
        
        let mut converted_tasks = Vec::new();
        
        for (tc_uuid, tc_task) in tc_tasks {
            // Convert TaskChampion task to task-hookrs format
            if let Some(task) = convert_taskchampion_to_hookrs(tc_uuid, tc_task)? {
                // Apply basic filtering if provided
                if filter.is_empty() || task_matches_filter(&task, filter) {
                    converted_tasks.push(task);
                }
            }
        }
        
        log::info!("TaskChampion backend: Exported {} tasks", converted_tasks.len());
        Ok(converted_tasks)
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

/// Simple filter matching - checks if task description contains the filter text
fn task_matches_filter(task: &Task, filter: &str) -> bool {
    if filter.is_empty() {
        return true;
    }
    
    // Convert to lowercase for case-insensitive matching
    let filter_lower = filter.to_lowercase();
    
    // Check description
    if task.description().to_lowercase().contains(&filter_lower) {
        return true;
    }
    
    // TODO: Add more sophisticated filtering (project, tags, etc.)
    
    false
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

