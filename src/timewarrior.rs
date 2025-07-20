use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{Duration, Instant},
    collections::HashSet,
    cell::RefCell,
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

/// Cache for tracking information
#[derive(Debug, Clone)]
struct TrackingCache {
    tracked_task_uuids: HashSet<String>,
    last_updated: Instant,
    cache_duration: Duration,
}

impl TrackingCache {
    fn new() -> Self {
        Self {
            tracked_task_uuids: HashSet::new(),
            last_updated: Instant::now() - Duration::from_secs(10), // Force initial refresh
            cache_duration: Duration::from_secs(5), // 5 second cache
        }
    }

    fn is_expired(&self) -> bool {
        self.last_updated.elapsed() > self.cache_duration
    }

    fn update(&mut self, tracked_uuids: HashSet<String>) {
        self.tracked_task_uuids = tracked_uuids;
        self.last_updated = Instant::now();
    }

    fn contains(&self, uuid: &str) -> bool {
        self.tracked_task_uuids.contains(uuid)
    }
}

/// Timewarrior integration handler
pub struct TimewarriorIntegration {
    config: TimewarriorConfig,
    task_hooks_dir: PathBuf,
    tracking_cache: RefCell<TrackingCache>,
}

impl TimewarriorIntegration {
    pub fn new() -> Result<Self> {
        let task_hooks_dir = Self::get_task_hooks_dir()?;
        let config = Self::load_config()?;
        
        Ok(Self {
            config,
            task_hooks_dir,
            tracking_cache: RefCell::new(TrackingCache::new()),
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

    /// Refresh the cache with all currently tracked task UUIDs
    fn refresh_tracking_cache(&self) -> Result<()> {
        if !Self::check_timewarrior_available() || !self.config.enabled {
            self.tracking_cache.borrow_mut().update(HashSet::new());
            return Ok(());
        }

        // Check if timewarrior is actively tracking
        let output = Command::new("timew")
            .arg("get")
            .arg("dom.active")
            .output()?;

        if !output.status.success() {
            self.tracking_cache.borrow_mut().update(HashSet::new());
            return Ok(());
        }

        let active = String::from_utf8_lossy(&output.stdout);
        if active.trim() != "1" {
            // No active tracking
            self.tracking_cache.borrow_mut().update(HashSet::new());
            return Ok(());
        }

        // Get all active tracking tags
        let tags_output = Command::new("timew")
            .arg("get")
            .arg("dom.active.tag")
            .output()?;

        if !tags_output.status.success() {
            self.tracking_cache.borrow_mut().update(HashSet::new());
            return Ok(());
        }

        let tags = String::from_utf8_lossy(&tags_output.stdout);
        let mut tracked_uuids = HashSet::new();
        
        // Parse all tags to find UUID references
        for tag in tags.split_whitespace() {
            if let Some(uuid) = tag.strip_prefix("uuid:") {
                tracked_uuids.insert(uuid.to_string());
            }
        }

        self.tracking_cache.borrow_mut().update(tracked_uuids);
        Ok(())
    }

    /// Check if a specific task is currently being tracked by TimeWarrior
    pub fn is_task_being_tracked(&self, task_uuid: &str) -> bool {
        if !Self::check_timewarrior_available() || !self.config.enabled {
            return false;
        }

        // Refresh cache if expired
        if self.tracking_cache.borrow().is_expired() {
            if let Err(e) = self.refresh_tracking_cache() {
                warn!("Failed to refresh TimeWarrior tracking cache: {}", e);
                return false;
            }
        }

        self.tracking_cache.borrow().contains(task_uuid)
    }

    /// Force refresh the tracking cache (useful after task start/stop operations)
    pub fn force_refresh_tracking_cache(&self) -> Result<()> {
        self.refresh_tracking_cache()
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
            tracking_cache: RefCell::new(TrackingCache::new()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_timewarrior_config_default_values() {
        let config = TimewarriorConfig::default();
        assert!(config.enabled);
        assert_eq!(config.tag_prefix, "");
        assert!(config.include_project);
        assert!(!config.include_description);
        assert_eq!(config.log_level, "info");
        assert!(!config.hook_installed);
    }

    #[test]
    fn test_get_setup_instructions_complete() {
        let integration = TimewarriorIntegration::default();
        let instructions = integration.get_setup_instructions();
        
        // Verify we get some instructions
        assert!(!instructions.is_empty());
        
        // Verify instructions contain status indicators
        let text = instructions.join("\n");
        assert!(text.contains("✅") || text.contains("❌"));
    }

    #[test]
    fn test_active_tracking_info() {
        let info = ActiveTrackingInfo {
            tags: "project:test uuid:123".to_string(),
            duration: "1h30m".to_string(),
        };
        
        assert_eq!(info.tags, "project:test uuid:123");
        assert_eq!(info.duration, "1h30m");
    }

    #[test]
    fn test_timewarrior_status_complete() {
        let status = TimewarriorStatus {
            timewarrior_available: true,
            hook_installed: true,
            integration_enabled: true,
            active_tracking: Some(ActiveTrackingInfo {
                tags: "work".to_string(),
                duration: "45m".to_string(),
            }),
        };
        
        assert!(status.timewarrior_available);
        assert!(status.hook_installed);
        assert!(status.integration_enabled);
        assert!(status.active_tracking.is_some());
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let original = TimewarriorConfig {
            enabled: false,
            tag_prefix: "tw_".to_string(),
            include_project: false,
            include_description: true,
            log_level: "debug".to_string(),
            hook_installed: true,
        };
        
        // Serialize
        let json = serde_json::to_string(&original).expect("Failed to serialize");
        
        // Deserialize
        let deserialized: TimewarriorConfig = 
            serde_json::from_str(&json).expect("Failed to deserialize");
        
        // Verify all fields match
        assert_eq!(original.enabled, deserialized.enabled);
        assert_eq!(original.tag_prefix, deserialized.tag_prefix);
        assert_eq!(original.include_project, deserialized.include_project);
        assert_eq!(original.include_description, deserialized.include_description);
        assert_eq!(original.log_level, deserialized.log_level);
        assert_eq!(original.hook_installed, deserialized.hook_installed);
    }

    #[test]
    fn test_get_task_hooks_dir_fallback() {
        // Test that get_task_hooks_dir returns a path
        let result = TimewarriorIntegration::get_task_hooks_dir();
        
        // Should either succeed or fail, but not panic
        if let Ok(path) = result {
            // If successful, path should end with hooks
            assert!(path.to_string_lossy().contains("hooks"));
        }
    }

    #[test]
    fn test_is_task_being_tracked_when_disabled() {
        let mut integration = TimewarriorIntegration::default();
        integration.config.enabled = false;
        
        // Should return false when integration is disabled
        assert!(!integration.is_task_being_tracked("some-uuid"));
    }

    #[test]
    fn test_is_task_being_tracked_when_enabled() {
        let integration = TimewarriorIntegration::default();
        
        // Should not panic when checking task tracking
        // (Result depends on whether timewarrior is available and what's being tracked)
        let _result = integration.is_task_being_tracked("test-uuid-123");
    }

    #[test]
    fn test_tracking_cache_functionality() {
        let integration = TimewarriorIntegration::default();
        
        // First call should attempt to refresh cache
        let result1 = integration.is_task_being_tracked("test-uuid-123");
        
        // Second call should use cache (if within cache duration)
        let result2 = integration.is_task_being_tracked("test-uuid-123");
        
        // Results should be consistent
        assert_eq!(result1, result2);
    }
}