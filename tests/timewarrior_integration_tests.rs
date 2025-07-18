use taskwarrior_tui::timewarrior::{TimewarriorConfig, TimewarriorIntegration, TimewarriorStatus, ActiveTrackingInfo};


#[test]
fn test_timewarrior_config_default() {
    let config = TimewarriorConfig::default();
    assert!(config.enabled);
    assert_eq!(config.tag_prefix, "");
    assert!(config.include_project);
    assert!(!config.include_description);
    assert_eq!(config.log_level, "info");
    assert!(!config.hook_installed);
}

#[test]
fn test_timewarrior_config_serialization() {
    let config = TimewarriorConfig {
        enabled: false,
        tag_prefix: "tw_".to_string(),
        include_project: false,
        include_description: true,
        log_level: "debug".to_string(),
        hook_installed: true,
    };

    // Test serialization
    let serialized = serde_json::to_string(&config).expect("Failed to serialize");
    let deserialized: TimewarriorConfig = serde_json::from_str(&serialized).expect("Failed to deserialize");
    
    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(config.tag_prefix, deserialized.tag_prefix);
    assert_eq!(config.include_project, deserialized.include_project);
    assert_eq!(config.include_description, deserialized.include_description);
    assert_eq!(config.log_level, deserialized.log_level);
    assert_eq!(config.hook_installed, deserialized.hook_installed);
}

#[test]
fn test_check_timewarrior_available() {
    // This test will pass or fail based on whether timewarrior is installed
    let available = TimewarriorIntegration::check_timewarrior_available();
    // Just verify the function runs without panic
    println!("Timewarrior available: {}", available);
}

#[test]
fn test_timewarrior_status_struct() {
    let status = TimewarriorStatus {
        timewarrior_available: true,
        hook_installed: false,
        integration_enabled: true,
        active_tracking: Some(ActiveTrackingInfo {
            tags: "project:test uuid:123".to_string(),
            duration: "1h23m".to_string(),
        }),
    };
    
    assert!(status.timewarrior_available);
    assert!(!status.hook_installed);
    assert!(status.integration_enabled);
    assert!(status.active_tracking.is_some());
    
    let tracking = status.active_tracking.unwrap();
    assert_eq!(tracking.tags, "project:test uuid:123");
    assert_eq!(tracking.duration, "1h23m");
}

#[test]
fn test_setup_instructions_no_timewarrior() {
    // Create a mock integration with specific config
    let integration = TimewarriorIntegration::default();
    let instructions = integration.get_setup_instructions();
    
    // Should have at least some instructions
    assert!(!instructions.is_empty());
    
    // Check that instructions contain expected markers
    let has_checkmark = instructions.iter().any(|s| s.contains("✅"));
    let has_cross = instructions.iter().any(|s| s.contains("❌"));
    assert!(has_checkmark || has_cross);
}

#[cfg(test)]
mod mock_tests {
    use super::*;
    
    /// Mock TimewarriorIntegration for testing without external dependencies
    struct MockTimewarriorIntegration {
        config: TimewarriorConfig,
        mock_timewarrior_available: bool,
        mock_active_tracking: Option<ActiveTrackingInfo>,
    }
    
    impl MockTimewarriorIntegration {
        fn new(available: bool, hook_installed: bool, enabled: bool) -> Self {
            let mut config = TimewarriorConfig::default();
            config.hook_installed = hook_installed;
            config.enabled = enabled;
            
            Self {
                config,
                mock_timewarrior_available: available,
                mock_active_tracking: None,
            }
        }
        
        fn get_status(&self) -> TimewarriorStatus {
            TimewarriorStatus {
                timewarrior_available: self.mock_timewarrior_available,
                hook_installed: self.config.hook_installed,
                integration_enabled: self.config.enabled,
                active_tracking: self.mock_active_tracking.clone(),
            }
        }
        
        fn get_setup_instructions(&self) -> Vec<String> {
            let mut instructions = Vec::new();
            
            if !self.mock_timewarrior_available {
                instructions.push("❌ Timewarrior not found. Please install timewarrior first:".to_string());
            } else {
                instructions.push("✅ Timewarrior found and available".to_string());
            }
            
            if !self.config.hook_installed {
                instructions.push("❌ Timewarrior hook not installed".to_string());
            } else {
                instructions.push("✅ Timewarrior hook installed".to_string());
            }
            
            if !self.config.enabled {
                instructions.push("❌ Timewarrior integration disabled".to_string());
            } else {
                instructions.push("✅ Timewarrior integration enabled".to_string());
            }
            
            instructions
        }
    }
    
    #[test]
    fn test_mock_integration_all_good() {
        let integration = MockTimewarriorIntegration::new(true, true, true);
        let status = integration.get_status();
        
        assert!(status.timewarrior_available);
        assert!(status.hook_installed);
        assert!(status.integration_enabled);
        assert!(status.active_tracking.is_none());
        
        let instructions = integration.get_setup_instructions();
        assert_eq!(instructions.len(), 3);
        assert!(instructions.iter().all(|s| s.contains("✅")));
    }
    
    #[test]
    fn test_mock_integration_no_timewarrior() {
        let integration = MockTimewarriorIntegration::new(false, false, true);
        let status = integration.get_status();
        
        assert!(!status.timewarrior_available);
        assert!(!status.hook_installed);
        assert!(status.integration_enabled);
        
        let instructions = integration.get_setup_instructions();
        assert!(instructions.iter().any(|s| s.contains("❌ Timewarrior not found")));
    }
    
    #[test]
    fn test_mock_integration_with_active_tracking() {
        let mut integration = MockTimewarriorIntegration::new(true, true, true);
        integration.mock_active_tracking = Some(ActiveTrackingInfo {
            tags: "test_project work".to_string(),
            duration: "2h15m".to_string(),
        });
        
        let status = integration.get_status();
        assert!(status.active_tracking.is_some());
        
        let tracking = status.active_tracking.unwrap();
        assert_eq!(tracking.tags, "test_project work");
        assert_eq!(tracking.duration, "2h15m");
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    
    #[test]
    fn test_config_update_flow() {
        let mut config = TimewarriorConfig::default();
        assert!(config.enabled);
        assert_eq!(config.tag_prefix, "");
        
        // Update config
        config.enabled = false;
        config.tag_prefix = "tw_".to_string();
        config.include_description = true;
        config.log_level = "debug".to_string();
        
        assert!(!config.enabled);
        assert_eq!(config.tag_prefix, "tw_");
        assert!(config.include_description);
        assert_eq!(config.log_level, "debug");
    }
    
    #[test]
    fn test_config_edge_cases() {
        let mut config = TimewarriorConfig::default();
        
        // Test empty tag prefix
        config.tag_prefix = "".to_string();
        assert_eq!(config.tag_prefix, "");
        
        // Test long tag prefix
        config.tag_prefix = "very_long_prefix_that_might_cause_issues_".to_string();
        assert_eq!(config.tag_prefix, "very_long_prefix_that_might_cause_issues_");
        
        // Test various log levels
        for level in &["trace", "debug", "info", "warn", "error"] {
            config.log_level = level.to_string();
            assert_eq!(config.log_level, *level);
        }
    }
}

#[cfg(test)]
mod integration_flow_tests {
    use super::*;
    
    #[test]
    fn test_full_integration_flow() {
        // Test the expected flow of timewarrior integration
        
        // 1. Check initial state
        let default_config = TimewarriorConfig::default();
        assert!(default_config.enabled);
        assert!(!default_config.hook_installed);
        
        // 2. Create status for uninstalled state
        let uninstalled_status = TimewarriorStatus {
            timewarrior_available: true,
            hook_installed: false,
            integration_enabled: true,
            active_tracking: None,
        };
        
        assert!(uninstalled_status.timewarrior_available);
        assert!(!uninstalled_status.hook_installed);
        assert!(uninstalled_status.active_tracking.is_none());
        
        // 3. Simulate installed state
        let installed_status = TimewarriorStatus {
            timewarrior_available: true,
            hook_installed: true,
            integration_enabled: true,
            active_tracking: Some(ActiveTrackingInfo {
                tags: "project:important uuid:abc123".to_string(),
                duration: "0h45m".to_string(),
            }),
        };
        
        assert!(installed_status.hook_installed);
        assert!(installed_status.active_tracking.is_some());
    }
    
    #[test]
    fn test_error_scenarios() {
        // Test various error scenarios
        
        // No timewarrior available
        let no_tw_status = TimewarriorStatus {
            timewarrior_available: false,
            hook_installed: false,
            integration_enabled: true,
            active_tracking: None,
        };
        
        assert!(!no_tw_status.timewarrior_available);
        assert!(no_tw_status.active_tracking.is_none());
        
        // Integration disabled
        let disabled_status = TimewarriorStatus {
            timewarrior_available: true,
            hook_installed: true,
            integration_enabled: false,
            active_tracking: None,
        };
        
        assert!(!disabled_status.integration_enabled);
    }
}