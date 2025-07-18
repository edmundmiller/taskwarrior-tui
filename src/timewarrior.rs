use std::{
    fs,
    path::PathBuf,
    process::Command,
};

use anyhow::{anyhow, Context, Result};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

/// Configuration for timewarrior integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimewarriorConfig {
    pub enabled: bool,
    pub tag_prefix: String,
    pub include_project: bool,
    pub include_description: bool,
    pub log_level: String,
    pub hook_installed: bool,
}

impl Default for TimewarriorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            tag_prefix: String::new(),
            include_project: true,
            include_description: false,
            log_level: "info".to_string(),
            hook_installed: false,
        }
    }
}

/// Timewarrior integration handler
pub struct TimewarriorIntegration {
    config: TimewarriorConfig,
    task_hooks_dir: PathBuf,
}

impl TimewarriorIntegration {
    pub fn new() -> Result<Self> {
        let task_hooks_dir = Self::get_task_hooks_dir()?;
        let config = Self::load_config()?;
        
        Ok(Self {
            config,
            task_hooks_dir,
        })
    }

    /// Get the taskwarrior hooks directory
    fn get_task_hooks_dir() -> Result<PathBuf> {
        // Try to get from taskwarrior configuration
        if let Ok(output) = Command::new("task")
            .arg("_get")
            .arg("rc.data.location")
            .output()
        {
            if output.status.success() {
                let data_location = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !data_location.is_empty() {
                    return Ok(PathBuf::from(data_location).join("hooks"));
                }
            }
        }

        // Fall back to default location
        let home = dirs::home_dir().ok_or_else(|| anyhow!("Unable to determine home directory"))?;
        Ok(home.join(".task").join("hooks"))
    }

    /// Load timewarrior configuration from taskwarrior
    fn load_config() -> Result<TimewarriorConfig> {
        let mut config = TimewarriorConfig::default();

        // Load configuration from taskwarrior UDA settings
        let config_keys = vec![
            ("enabled", "true"),
            ("tag.prefix", ""),
            ("include.project", "true"),
            ("include.description", "false"),
            ("log.level", "info"),
        ];

        for (key, default_value) in config_keys {
            if let Ok(output) = Command::new("task")
                .arg("_get")
                .arg(format!("rc.uda.timewarrior.{}", key))
                .output()
            {
                if output.status.success() {
                    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !value.is_empty() {
                        match key {
                            "enabled" => config.enabled = value.to_lowercase() == "true",
                            "tag.prefix" => config.tag_prefix = value,
                            "include.project" => config.include_project = value.to_lowercase() == "true",
                            "include.description" => config.include_description = value.to_lowercase() == "true",
                            "log.level" => config.log_level = value,
                            _ => {}
                        }
                    } else {
                        // Set default values if not configured
                        match key {
                            "enabled" => config.enabled = default_value == "true",
                            "include.project" => config.include_project = default_value == "true",
                            "include.description" => config.include_description = default_value == "true",
                            _ => {}
                        }
                    }
                }
            }
        }

        // Check if hook is installed
        let hooks_dir = Self::get_task_hooks_dir()?;
        config.hook_installed = hooks_dir.join("on-modify.timewarrior").exists();

        Ok(config)
    }

    /// Check if timewarrior is available
    pub fn check_timewarrior_available() -> bool {
        Command::new("timew")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Install the timewarrior hook
    pub fn install_hook(&self) -> Result<()> {
        // Create hooks directory if it doesn't exist
        fs::create_dir_all(&self.task_hooks_dir)
            .with_context(|| format!("Failed to create hooks directory: {:?}", self.task_hooks_dir))?;

        // Get the hook source path from the current executable directory
        let hook_source = Self::get_hook_source_path()?;
        let hook_dest = self.task_hooks_dir.join("on-modify.timewarrior");

        // Copy the hook file
        fs::copy(&hook_source, &hook_dest)
            .with_context(|| format!("Failed to copy hook from {:?} to {:?}", hook_source, hook_dest))?;

        // Make the hook executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_dest)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_dest, perms)?;
        }

        info!("Timewarrior hook installed successfully to {:?}", hook_dest);
        Ok(())
    }

    /// Get the path to the hook source file
    fn get_hook_source_path() -> Result<PathBuf> {
        // Try to find the hook in the current directory structure
        let current_dir = std::env::current_dir()?;
        let hook_path = current_dir.join("hooks").join("on-modify.timewarrior");
        
        if hook_path.exists() {
            return Ok(hook_path);
        }

        // Try relative to the executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let hook_path = exe_dir.join("hooks").join("on-modify.timewarrior");
                if hook_path.exists() {
                    return Ok(hook_path);
                }
            }
        }

        Err(anyhow!("Unable to find hook source file"))
    }

    /// Uninstall the timewarrior hook
    pub fn uninstall_hook(&self) -> Result<()> {
        let hook_path = self.task_hooks_dir.join("on-modify.timewarrior");
        
        if hook_path.exists() {
            fs::remove_file(&hook_path)
                .with_context(|| format!("Failed to remove hook file: {:?}", hook_path))?;
            info!("Timewarrior hook uninstalled successfully");
        } else {
            warn!("Hook file not found: {:?}", hook_path);
        }

        Ok(())
    }

    /// Check if the hook is installed
    pub fn is_hook_installed(&self) -> bool {
        self.config.hook_installed
    }

    /// Get the current configuration
    pub fn get_config(&self) -> &TimewarriorConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, new_config: TimewarriorConfig) -> Result<()> {
        self.config = new_config;
        self.save_config()
    }

    /// Save configuration to taskwarrior
    fn save_config(&self) -> Result<()> {
        let config_updates = vec![
            ("enabled", self.config.enabled.to_string()),
            ("tag.prefix", self.config.tag_prefix.clone()),
            ("include.project", self.config.include_project.to_string()),
            ("include.description", self.config.include_description.to_string()),
            ("log.level", self.config.log_level.clone()),
        ];

        for (key, value) in config_updates {
            let result = Command::new("task")
                .arg("config")
                .arg(format!("uda.timewarrior.{}", key))
                .arg(&value)
                .output();

            if let Err(e) = result {
                error!("Failed to save config key '{}': {}", key, e);
            }
        }

        Ok(())
    }

    /// Get timewarrior status information
    pub fn get_status(&self) -> TimewarriorStatus {
        TimewarriorStatus {
            timewarrior_available: Self::check_timewarrior_available(),
            hook_installed: self.is_hook_installed(),
            integration_enabled: self.config.enabled,
            active_tracking: self.get_active_tracking_info(),
        }
    }

    /// Get active tracking information from timewarrior
    fn get_active_tracking_info(&self) -> Option<ActiveTrackingInfo> {
        if !Self::check_timewarrior_available() {
            return None;
        }

        // Check if timewarrior is actively tracking
        let output = Command::new("timew")
            .arg("get")
            .arg("dom.active")
            .output()
            .ok()?;

        if output.status.success() {
            let active = String::from_utf8_lossy(&output.stdout);
            if active.trim() == "1" {
                // Get active tracking details
                let tags_output = Command::new("timew")
                    .arg("get")
                    .arg("dom.active.tag.1")
                    .output()
                    .ok()?;

                let duration_output = Command::new("timew")
                    .arg("get")
                    .arg("dom.active.duration")
                    .output()
                    .ok()?;

                let tags = if tags_output.status.success() {
                    String::from_utf8_lossy(&tags_output.stdout).trim().to_string()
                } else {
                    "No tags".to_string()
                };

                let duration = if duration_output.status.success() {
                    String::from_utf8_lossy(&duration_output.stdout).trim().to_string()
                } else {
                    "Unknown".to_string()
                };

                return Some(ActiveTrackingInfo {
                    tags,
                    duration,
                });
            }
        }

        None
    }

    /// Generate setup instructions for the user
    pub fn get_setup_instructions(&self) -> Vec<String> {
        let mut instructions = Vec::new();

        if !Self::check_timewarrior_available() {
            instructions.push("❌ Timewarrior not found. Please install timewarrior first:".to_string());
            instructions.push("   - Linux: apt install timewarrior / yum install timewarrior".to_string());
            instructions.push("   - macOS: brew install timewarrior".to_string());
            instructions.push("   - Windows: See https://timewarrior.net/download/".to_string());
            instructions.push("".to_string());
        } else {
            instructions.push("✅ Timewarrior found and available".to_string());
        }

        if !self.is_hook_installed() {
            instructions.push("❌ Timewarrior hook not installed".to_string());
            instructions.push("   Run the hook installation command to enable integration".to_string());
        } else {
            instructions.push("✅ Timewarrior hook installed".to_string());
        }

        if !self.config.enabled {
            instructions.push("❌ Timewarrior integration disabled".to_string());
            instructions.push("   Enable integration in configuration".to_string());
        } else {
            instructions.push("✅ Timewarrior integration enabled".to_string());
        }

        instructions
    }
}

/// Status information for timewarrior integration
#[derive(Debug, Clone)]
pub struct TimewarriorStatus {
    pub timewarrior_available: bool,
    pub hook_installed: bool,
    pub integration_enabled: bool,
    pub active_tracking: Option<ActiveTrackingInfo>,
}

/// Information about active timewarrior tracking
#[derive(Debug, Clone)]
pub struct ActiveTrackingInfo {
    pub tags: String,
    pub duration: String,
}

impl Default for TimewarriorIntegration {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            config: TimewarriorConfig::default(),
            task_hooks_dir: PathBuf::from("~/.task/hooks"),
        })
    }
}