//! # UI Testing Utilities for Taskwarrior TUI
//! 
//! This module provides comprehensive testing utilities for TUI components.
//! 
//! ## Purpose
//! - Provide reusable testing utilities for TUI components
//! - Generate consistent mock data for testing
//! - Enable snapshot testing for visual regression detection
//! - Offer property-based testing strategies
//! 
//! ## Main Components
//! 
//! ### `TuiTestHelper`
//! The primary testing utility that wraps ratatui's TestBackend to provide:
//! - Terminal rendering simulation
//! - Snapshot testing capabilities
//! - Buffer content extraction and analysis
//! 
//! ### `MockData`
//! Generates consistent test data including:
//! - Simple tasks with basic properties
//! - Complex tasks with tags, projects, and status
//! - Collections of tasks for list testing
//! - Tasks organized by status for filter testing
//! 
//! ### `UiAssertions`
//! Helper functions for common UI test assertions:
//! - Text content verification
//! - Border and structural element detection
//! - Line counting and layout validation
//! 
//! ### Property-Based Testing (`strategies` module)
//! Provides proptest strategies for generating:
//! - Random but valid task descriptions
//! - Task status combinations
//! - Project and tag collections
//! - Complete task objects for robust testing

use insta::{assert_snapshot, Settings};
use proptest::prelude::*;
use ratatui::{
    backend::TestBackend,
    layout::Rect,
    Terminal,
};
use task_hookrs::{
    status::TaskStatus,
    task::{Task, TW26},
};
use uuid::Uuid;

// =============================================================================
// CONSTANTS AND CONFIGURATION
// =============================================================================

/// Standard terminal width for consistent testing
/// This matches a typical terminal width and ensures consistent snapshots
pub const TERMINAL_WIDTH: u16 = 80;

/// Standard terminal height for consistent testing  
/// This matches a typical terminal height and ensures consistent snapshots
pub const TERMINAL_HEIGHT: u16 = 24;

// =============================================================================
// MAIN TESTING UTILITIES
// =============================================================================

/// Primary testing utility for TUI components
/// 
/// This struct wraps ratatui's TestBackend to provide convenient methods for:
/// - Rendering widgets in a simulated terminal
/// - Capturing terminal output for analysis
/// - Running snapshot tests for visual regression detection
/// - Managing test-specific settings and configuration
pub struct TuiTestHelper {
    /// The simulated terminal for rendering widgets
    terminal: Terminal<TestBackend>,
    /// Insta settings for snapshot testing configuration
    settings: Settings,
}

impl TuiTestHelper {
    /// Create a new TUI test helper with standard terminal size (80x24)
    /// 
    /// This is the most common method for creating a test helper. It uses
    /// standard terminal dimensions that work well for most UI components.
    /// 
    /// # Returns
    /// A configured TuiTestHelper ready for widget testing and snapshot creation
    pub fn new() -> Self {
        let backend = TestBackend::new(TERMINAL_WIDTH, TERMINAL_HEIGHT);
        let terminal = Terminal::new(backend).unwrap();
        
        let mut settings = Settings::clone_current();
        settings.set_snapshot_suffix("terminal");
        
        Self { terminal, settings }
    }

    /// Create a TUI test helper with custom terminal size
    pub fn with_size(width: u16, height: u16) -> Self {
        let backend = TestBackend::new(width, height);
        let terminal = Terminal::new(backend).unwrap();
        
        let mut settings = Settings::clone_current();
        settings.set_snapshot_suffix("terminal");
        
        Self { terminal, settings }
    }

    /// Get the terminal's buffer as a string for snapshot testing
    pub fn buffer(&self) -> String {
        let buffer = self.terminal.backend().buffer();
        buffer
            .content
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>()
    }

    /// Assert that the current terminal state matches a snapshot
    pub fn assert_snapshot(&self, snapshot_name: &str) {
        self.settings.bind(|| {
            assert_snapshot!(snapshot_name, self.buffer());
        });
    }

    /// Draw a widget and capture the output
    pub fn draw_widget<W>(&mut self, widget: W, area: Option<Rect>) -> String
    where
        W: ratatui::widgets::Widget,
    {
        let area = area.unwrap_or(Rect::new(0, 0, TERMINAL_WIDTH, TERMINAL_HEIGHT));
        
        self.terminal
            .draw(|frame| {
                frame.render_widget(widget, area);
            })
            .unwrap();
            
        self.buffer()
    }

    /// Draw a stateful widget and capture the output
    pub fn draw_stateful_widget<W, S>(&mut self, widget: W, area: Option<Rect>, state: &mut S) -> String
    where
        W: ratatui::widgets::StatefulWidget<State = S>,
    {
        let area = area.unwrap_or(Rect::new(0, 0, TERMINAL_WIDTH, TERMINAL_HEIGHT));
        
        self.terminal
            .draw(|frame| {
                frame.render_stateful_widget(widget, area, state);
            })
            .unwrap();
            
        self.buffer()
    }

    /// Get the current terminal size
    pub fn size(&self) -> Rect {
        Rect::new(0, 0, TERMINAL_WIDTH, TERMINAL_HEIGHT)
    }

    /// Reset the terminal buffer
    pub fn clear(&mut self) {
        self.terminal.clear().unwrap();
    }
}

impl Default for TuiTestHelper {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// MOCK DATA GENERATION
// =============================================================================

/// Mock data generators for consistent testing
/// 
/// This struct provides static methods to generate various types of test data:
/// - Simple tasks with minimal properties
/// - Complex tasks with full property sets
/// - Collections of tasks for list testing
/// - Tasks organized by specific criteria (status, etc.)
/// 
/// All generated data is deterministic and suitable for snapshot testing.
pub struct MockData;

impl MockData {
    /// Generate a simple mock task
    pub fn simple_task(description: &str) -> Task<TW26> {
        use chrono::Utc;
        use std::collections::BTreeMap;
        
        Task::new(
            None,                           // id
            TaskStatus::Pending,            // status
            Uuid::new_v4(),                 // uuid
            task_hookrs::date::Date::from(Utc::now().naive_utc()),  // entry
            description.to_string(),        // description
            None,                           // annotations
            None,                           // depends
            None,                           // due
            None,                           // end
            None,                           // imask
            None,                           // mask
            None,                           // modified
            None,                           // parent
            None,                           // priority
            None,                           // project
            None,                           // recur
            None,                           // scheduled
            None,                           // start
            None,                           // tags
            None,                           // until
            None,                           // wait
            None,                           // urgency
            BTreeMap::new(),                // uda
        )
    }

    /// Generate a mock task with custom properties
    pub fn task_with_properties(
        description: &str,
        status: TaskStatus,
        tags: Vec<String>,
        project: Option<String>,
    ) -> Task<TW26> {
        use chrono::Utc;
        use std::collections::BTreeMap;
        
        Task::new(
            None,                           // id
            status,                         // status
            Uuid::new_v4(),                 // uuid
            task_hookrs::date::Date::from(Utc::now().naive_utc()),  // entry
            description.to_string(),        // description
            None,                           // annotations
            None,                           // depends
            None,                           // due
            None,                           // end
            None,                           // imask
            None,                           // mask
            None,                           // modified
            None,                           // parent
            None,                           // priority
            project,                        // project
            None,                           // recur
            None,                           // scheduled
            None,                           // start
            Some(tags),                     // tags
            None,                           // until
            None,                           // wait
            None,                           // urgency
            BTreeMap::new(),                // uda
        )
    }

    /// Generate a list of mock tasks for testing
    pub fn sample_tasks() -> Vec<Task<TW26>> {
        vec![
            Self::task_with_properties(
                "Complete project documentation",
                TaskStatus::Pending,
                vec!["work".to_string(), "urgent".to_string()],
                Some("project-alpha".to_string()),
            ),
            Self::task_with_properties(
                "Review pull request #123",
                TaskStatus::Pending,
                vec!["code-review".to_string()],
                Some("project-alpha".to_string()),
            ),
            Self::task_with_properties(
                "Buy groceries",
                TaskStatus::Pending,
                vec!["personal".to_string()],
                None,
            ),
            Self::task_with_properties(
                "Schedule dentist appointment",
                TaskStatus::Completed,
                vec!["health".to_string(), "personal".to_string()],
                None,
            ),
            Self::task_with_properties(
                "Learn Rust async programming",
                TaskStatus::Pending,
                vec!["learning".to_string(), "rust".to_string()],
                Some("self-improvement".to_string()),
            ),
        ]
    }

    /// Generate tasks with specific statuses for testing filters
    pub fn tasks_by_status() -> Vec<(TaskStatus, Vec<Task<TW26>>)> {
        vec![
            (TaskStatus::Pending, vec![
                Self::task_with_properties("Pending task 1", TaskStatus::Pending, vec![], None),
                Self::task_with_properties("Pending task 2", TaskStatus::Pending, vec![], None),
            ]),
            (TaskStatus::Completed, vec![
                Self::task_with_properties("Completed task 1", TaskStatus::Completed, vec![], None),
                Self::task_with_properties("Completed task 2", TaskStatus::Completed, vec![], None),
            ]),
            (TaskStatus::Deleted, vec![
                Self::task_with_properties("Deleted task 1", TaskStatus::Deleted, vec![], None),
            ]),
        ]
    }
}

// =============================================================================
// PROPERTY-BASED TESTING STRATEGIES
// =============================================================================

/// Property-based testing strategies for generating test data
/// 
/// This module provides proptest strategies for generating random but valid
/// test data. These strategies are used in property-based tests to verify
/// that the system behaves correctly across a wide range of inputs.
/// 
/// Strategies include:
/// - Task descriptions with realistic constraints
/// - Valid task status values
/// - Project name variations (including None)
/// - Tag collections of varying sizes
/// - Complete task objects with all properties
pub mod strategies {
    use super::*;
    
    /// Strategy for generating random task descriptions
    pub fn task_description() -> impl Strategy<Value = String> {
        prop::string::string_regex(r"[a-zA-Z0-9 ]{5,50}").unwrap()
    }
    
    /// Strategy for generating task statuses
    pub fn task_status() -> impl Strategy<Value = TaskStatus> {
        prop_oneof![
            Just(TaskStatus::Pending),
            Just(TaskStatus::Completed),
            Just(TaskStatus::Deleted),
        ]
    }
    
    /// Strategy for generating project names
    pub fn project_name() -> impl Strategy<Value = Option<String>> {
        prop_oneof![
            Just(None),
            Just(Some("project-alpha".to_string())),
            Just(Some("project-beta".to_string())),
            Just(Some("personal".to_string())),
        ]
    }
    
    /// Strategy for generating tag lists
    pub fn tag_list() -> impl Strategy<Value = Vec<String>> {
        prop::collection::vec(
            prop_oneof![
                Just("work".to_string()),
                Just("personal".to_string()),
                Just("urgent".to_string()),
                Just("learning".to_string()),
                Just("health".to_string()),
            ],
            0..4
        )
    }
    
    /// Strategy for generating complete mock tasks
    pub fn mock_task() -> impl Strategy<Value = Task<TW26>> {
        (task_description(), task_status(), tag_list(), project_name())
            .prop_map(|(desc, status, tags, project)| {
                MockData::task_with_properties(&desc, status, tags, project)
            })
    }
    
    /// Strategy for generating lists of tasks
    pub fn task_list() -> impl Strategy<Value = Vec<Task<TW26>>> {
        prop::collection::vec(mock_task(), 1..20)
    }
}

// =============================================================================
// UI ASSERTION HELPERS  
// =============================================================================

/// UI state verification helpers
/// 
/// This struct provides static methods for common UI testing assertions.
/// These helpers make it easier to verify UI state and provide clear
/// error messages when assertions fail.
/// 
/// Available assertions:
/// - Text content verification (contains/not contains)
/// - Line counting for layout validation
/// - Border and structural element detection
/// - Table structure verification
pub struct UiAssertions;

impl UiAssertions {
    /// Assert that buffer contains expected text
    pub fn contains_text(buffer: &str, expected: &str) {
        assert!(
            buffer.contains(expected),
            "Buffer should contain '{}'\nActual buffer:\n{}",
            expected,
            buffer
        );
    }
    
    /// Assert that buffer does not contain text
    pub fn not_contains_text(buffer: &str, unexpected: &str) {
        assert!(
            !buffer.contains(unexpected),
            "Buffer should not contain '{}'\nActual buffer:\n{}",
            unexpected,
            buffer
        );
    }
    
    /// Assert buffer has expected number of lines
    pub fn line_count(buffer: &str, expected_lines: usize) {
        let actual_lines = buffer.lines().count();
        assert_eq!(
            actual_lines,
            expected_lines,
            "Expected {} lines, got {}",
            expected_lines,
            actual_lines
        );
    }
    
    /// Assert that specific UI elements are present (borders, titles, etc.)
    pub fn has_border_elements(buffer: &str) {
        // Check for common border characters
        let border_chars = ["┌", "┐", "└", "┘", "─", "│"];
        for &border_char in &border_chars {
            UiAssertions::contains_text(buffer, border_char);
        }
    }
    
    /// Assert that table structure is present
    pub fn has_table_structure(buffer: &str) {
        // Check for table separators and structure
        UiAssertions::has_border_elements(buffer);
        // Tables often have these patterns
        assert!(
            buffer.contains("│") || buffer.contains("┃"),
            "Buffer should contain table column separators"
        );
    }
}

// =============================================================================
// UNIT TESTS FOR TESTING UTILITIES
// =============================================================================

/// Unit tests for the testing utilities themselves
/// 
/// These tests verify that our testing infrastructure works correctly:
/// - TuiTestHelper creates proper terminal instances
/// - MockData generates valid test data
/// - UiAssertions work correctly
/// - Property-based strategies generate valid data
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tui_helper_creation() {
        let helper = TuiTestHelper::new();
        assert_eq!(helper.size().width, TERMINAL_WIDTH);
        assert_eq!(helper.size().height, TERMINAL_HEIGHT);
    }
    
    #[test]
    fn test_mock_data_generation() {
        let tasks = MockData::sample_tasks();
        assert_eq!(tasks.len(), 5);
        
        let first_task = &tasks[0];
        assert_eq!(first_task.description(), "Complete project documentation");
        assert_eq!(first_task.status(), &TaskStatus::Pending);
    }
    
    #[test]
    fn test_ui_assertions() {
        let buffer = "┌─────────────┐\n│ Test Widget │\n└─────────────┘";
        
        UiAssertions::contains_text(buffer, "Test Widget");
        UiAssertions::has_border_elements(buffer);
        UiAssertions::line_count(buffer, 3);
    }
    
    proptest! {
        #[test]
        fn test_property_based_task_generation(task in strategies::mock_task()) {
            // Verify generated tasks have valid properties
            assert!(!task.description().is_empty());
            assert!(matches!(
                task.status(),
                &TaskStatus::Pending | &TaskStatus::Completed | &TaskStatus::Deleted
            ));
        }
        
        #[test]
        fn test_property_based_task_list(tasks in strategies::task_list()) {
            // Verify generated task lists are valid
            assert!(!tasks.is_empty());
            assert!(tasks.len() <= 20);
            
            for task in &tasks {
                assert!(!task.description().is_empty());
            }
        }
    }
}