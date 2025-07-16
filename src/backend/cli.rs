use anyhow::Result;
use task_hookrs::{import::import, task::Task};
use uuid::Uuid;
use versions::Versioning;

use super::TaskBackend;

/// CLI-based task backend that shells out to the task command
pub struct CliBackend {
    task_version: Versioning,
}

impl CliBackend {
    pub fn new() -> Result<Self> {
        // Get taskwarrior version (extracted from original app.rs logic)
        let task_version = get_taskwarrior_version()?;
        log::debug!("Detected TaskWarrior version: {}", task_version);
        
        Ok(Self { task_version })
    }
}

impl TaskBackend for CliBackend {
    fn export_tasks(&self, filter: &str, report: &str, context_filter: &str) -> Result<Vec<Task>> {
        let mut task = std::process::Command::new("task");

        task
            .arg("rc.json.array=on")
            .arg("rc.confirmation=off")
            .arg("rc.json.depends.array=on")
            .arg("rc.color=off")
            .arg("rc._forcecolor=off");

        // Only add filter override if filter is not empty
        if !filter.trim().is_empty() {
            if let Some(args) = shlex::split(format!(r#"rc.report.{}.filter='{}'"#, report, filter.trim()).trim()) {
                for arg in args {
                    task.arg(arg);
                }
            }
        }

        let taskwarrior_version_supported = Versioning::new("3.0.0").unwrap();
        log::debug!("TaskWarrior version: {}, supported version: {}, comparison: {}", 
                   self.task_version, taskwarrior_version_supported, 
                   self.task_version >= taskwarrior_version_supported);
        
        if !context_filter.trim().is_empty() && self.task_version >= taskwarrior_version_supported {
            if let Some(args) = shlex::split(context_filter) {
                for arg in args {
                    task.arg(arg);
                }
            }
        } else if !context_filter.trim().is_empty() {
            task.arg(format!("'\\({}\\)'", context_filter));
        }

        task.arg("export");

        if self.task_version >= taskwarrior_version_supported {
            task.arg(report);
        }

        // Debug: Log the actual command being run
        let command_args: Vec<&std::ffi::OsStr> = task.get_args().collect();
        log::debug!("Running command: task {:?}", command_args);

        let output = task.output()?;
        let data = String::from_utf8_lossy(&output.stdout);

        if output.status.success() {
            let imported = import(data.as_bytes())?;
            Ok(imported)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Task export failed: {}", error))
        }
    }

    fn add_task(&self, description: &str, args: &[&str]) -> Result<()> {
        let mut cmd = std::process::Command::new("task");
        cmd.arg("add").arg(description);
        
        for arg in args {
            cmd.arg(arg);
        }
        
        let output = cmd.output()?;
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Task add failed: {}", error));
        }
        
        Ok(())
    }

    fn mark_done(&self, task_uuids: &[Uuid]) -> Result<()> {
        let mut cmd = std::process::Command::new("task");
        cmd
            .arg("rc.bulk=0")
            .arg("rc.confirmation=off")
            .arg("rc.dependency.confirmation=off")
            .arg("rc.recurrence.confirmation=off");
        
        for task_uuid in task_uuids {
            cmd.arg(task_uuid.to_string());
        }
        cmd.arg("done");
        
        let output = cmd.output()?;
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Task done failed: {}", error));
        }
        
        Ok(())
    }

    fn delete_tasks(&self, task_uuids: &[Uuid]) -> Result<()> {
        let mut cmd = std::process::Command::new("task");
        cmd
            .arg("rc.bulk=0")
            .arg("rc.confirmation=off")
            .arg("rc.dependency.confirmation=off")
            .arg("rc.recurrence.confirmation=off");
        
        for task_uuid in task_uuids {
            cmd.arg(task_uuid.to_string());
        }
        cmd.arg("delete");
        
        let output = cmd.output()?;
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Task delete failed: {}", error));
        }
        
        Ok(())
    }

    fn modify_tasks(&self, task_uuids: &[Uuid], modifications: &str) -> Result<()> {
        let mut cmd = std::process::Command::new("task");
        cmd
            .arg("rc.bulk=0")
            .arg("rc.confirmation=off")
            .arg("rc.dependency.confirmation=off")
            .arg("rc.recurrence.confirmation=off");
        
        for task_uuid in task_uuids {
            cmd.arg(task_uuid.to_string());
        }
        
        if let Some(args) = shlex::split(modifications) {
            for arg in args {
                cmd.arg(arg);
            }
        }
        
        let output = cmd.output()?;
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Task modify failed: {}", error));
        }
        
        Ok(())
    }

    fn get_task_details(&self, task_uuid: Uuid) -> Result<Option<String>> {
        let mut cmd = std::process::Command::new("task");
        cmd
            .arg("rc.json.array=on")
            .arg("rc.confirmation=off")
            .arg("rc.json.depends.array=on")
            .arg("rc.color=off")
            .arg("rc._forcecolor=off")
            .arg(task_uuid.to_string())
            .arg("export");
        
        let output = cmd.output()?;
        if output.status.success() {
            let data = String::from_utf8_lossy(&output.stdout);
            Ok(Some(data.to_string()))
        } else {
            Ok(None)
        }
    }

    fn sync(&self) -> Result<()> {
        let mut cmd = std::process::Command::new("task");
        cmd.arg("sync");
        
        let output = cmd.output()?;
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Task sync failed: {}", error));
        }
        
        Ok(())
    }
}

fn get_taskwarrior_version() -> Result<Versioning> {
    let output = std::process::Command::new("task").arg("--version").output()?;
    
    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get taskwarrior version"));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let version_line = stdout.lines().next().unwrap_or("");
    
    // Extract version from output like "task 2.6.2 (2022-10-19)" or "task 3.0.0" or just "3.4.1"
    let version_str = if version_line.starts_with("task ") {
        // Format: "task 2.6.2" or "task 2.6.2 (2022-10-19)"
        version_line
            .split_whitespace()
            .nth(1)
            .unwrap_or("0.0.0")
    } else {
        // Format: just "3.4.1"
        version_line.trim()
    };
    
    Versioning::new(version_str).ok_or_else(|| anyhow::anyhow!("Failed to parse version: {}", version_str))
}