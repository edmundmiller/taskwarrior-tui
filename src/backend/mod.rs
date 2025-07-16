use anyhow::Result;
use task_hookrs::task::Task;
use uuid::Uuid;

pub mod cli;
#[cfg(feature = "taskchampion-backend")]
pub mod taskchampion;

#[cfg(test)]
mod tests;

/// Task backend trait for different task storage implementations
pub trait TaskBackend {
    /// Export tasks based on filter and report
    fn export_tasks(&self, filter: &str, report: &str, context_filter: &str) -> Result<Vec<Task>>;
    
    /// Add a new task with the given description and args
    fn add_task(&self, description: &str, args: &[&str]) -> Result<()>;
    
    /// Mark task(s) as done
    fn mark_done(&self, task_uuids: &[Uuid]) -> Result<()>;
    
    /// Delete task(s)
    fn delete_tasks(&self, task_uuids: &[Uuid]) -> Result<()>;
    
    /// Modify task(s) with given modifications
    fn modify_tasks(&self, task_uuids: &[Uuid], modifications: &str) -> Result<()>;
    
    /// Get task details for a specific task
    fn get_task_details(&self, task_uuid: Uuid) -> Result<Option<String>>;
    
    /// Sync tasks (for backends that support it)
    fn sync(&self) -> Result<()>;
}

/// Backend configuration enum
#[derive(Debug, Clone)]
pub enum BackendConfig {
    /// Use the traditional CLI backend
    Cli,
    /// Use the TaskChampion backend
    #[cfg(feature = "taskchampion-backend")]
    TaskChampion {
        /// Directory for TaskChampion database
        data_dir: Option<std::path::PathBuf>,
        /// Server configuration for sync
        server_config: Option<String>,
    },
}

impl Default for BackendConfig {
    fn default() -> Self {
        #[cfg(feature = "taskchampion-backend")]
        {
            Self::TaskChampion {
                data_dir: None,
                server_config: None,
            }
        }
        #[cfg(not(feature = "taskchampion-backend"))]
        {
            Self::Cli
        }
    }
}

/// Create a backend instance from configuration
pub fn create_backend(config: BackendConfig) -> Result<Box<dyn TaskBackend>> {
    match config {
        BackendConfig::Cli => {
            Ok(Box::new(cli::CliBackend::new()?))
        }
        #[cfg(feature = "taskchampion-backend")]
        BackendConfig::TaskChampion { data_dir, server_config } => {
            Ok(Box::new(taskchampion::TaskChampionBackend::new(data_dir, server_config)?))
        }
    }
}