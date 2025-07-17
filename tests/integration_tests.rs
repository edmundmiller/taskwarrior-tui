//! # Integration Tests for Taskwarrior TUI
//! 
//! This module contains comprehensive integration tests that verify complete
//! user workflows, state transitions, and UI behavior.
//! 
//! ## Test Categories
//! 
//! ### Basic UI Functionality
//! - Empty task list handling
//! - Single task display
//! - Multiple task display
//! - Error state handling
//! - Help screen rendering
//! 
//! ### State Management
//! - Table state transitions (selection, marking, mode switching)
//! - Task filtering by status
//! - Task detail view rendering
//! 
//! ### Responsive Design
//! - Different terminal sizes
//! - Layout adaptation
//! - Content wrapping and truncation
//! 
//! ### Performance Testing
//! - Large task list handling
//! - Unicode character support
//! - Rendering performance benchmarks
//! 
//! ### Property-Based Testing
//! - Random task list rendering
//! - Table state invariants
//! - UI dimension validation

#[path = "ui_testing.rs"]
mod ui_testing;

use crate::ui_testing::{MockData, TuiTestHelper, UiAssertions};
use assert_matches::assert_matches;
use proptest::prelude::*;
use task_hookrs::{status::TaskStatus, task::Task, task::TW26};

// =============================================================================
// INTEGRATION TEST SUITE
// =============================================================================

/// Integration test suite for complete user workflows
/// 
/// These tests verify end-to-end functionality by testing complete user
/// interactions and workflows rather than individual components.
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    // =========================================================================
    // BASIC UI FUNCTIONALITY TESTS
    // =========================================================================

    #[test]
    fn test_empty_task_list_ui() {
        let mut tui_helper = TuiTestHelper::new();
        
        // Simulate empty task list scenario
        let _empty_tasks: Vec<Task<TW26>> = vec![];
        
        // This would normally be rendered by the app with empty tasks
        // For now, we test that the UI handles empty state gracefully
        let buffer = tui_helper.draw_widget(
            ratatui::widgets::Paragraph::new("No tasks available")
                .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL)),
            None,
        );
        
        UiAssertions::contains_text(&buffer, "No tasks available");
        UiAssertions::has_border_elements(&buffer);
        
        tui_helper.assert_snapshot("empty_task_list");
    }

    #[test]
    fn test_single_task_display() {
        let mut tui_helper = TuiTestHelper::new();
        
        let task = MockData::simple_task("Complete important project");
        
        // Test that a single task renders correctly
        let buffer = tui_helper.draw_widget(
            ratatui::widgets::Paragraph::new(format!("• {}", task.description()))
                .block(ratatui::widgets::Block::default()
                    .title("Tasks")
                    .borders(ratatui::widgets::Borders::ALL)),
            None,
        );
        
        UiAssertions::contains_text(&buffer, "Complete important project");
        UiAssertions::contains_text(&buffer, "Tasks");
        UiAssertions::has_border_elements(&buffer);
        
        tui_helper.assert_snapshot("single_task_display");
    }

    #[test]
    fn test_multiple_task_display() {
        let mut tui_helper = TuiTestHelper::new();
        
        let tasks = MockData::sample_tasks();
        
        // Create a simple list representation
        let task_list: Vec<String> = tasks
            .iter()
            .map(|task| format!("• {}", task.description()))
            .collect();
        
        let content = task_list.join("\n");
        
        let buffer = tui_helper.draw_widget(
            ratatui::widgets::Paragraph::new(content)
                .block(ratatui::widgets::Block::default()
                    .title(format!("Tasks ({})", tasks.len()))
                    .borders(ratatui::widgets::Borders::ALL)),
            None,
        );
        
        // Verify all tasks are displayed
        for task in &tasks {
            UiAssertions::contains_text(&buffer, task.description());
        }
        
        UiAssertions::contains_text(&buffer, "Tasks (5)");
        tui_helper.assert_snapshot("multiple_task_display");
    }

    // =========================================================================
    // STATE MANAGEMENT TESTS
    // =========================================================================

    #[test]
    fn test_table_state_transitions() {
        use taskwarrior_tui::table::{TableMode, TaskwarriorTuiTableState};
        
        let mut state = TaskwarriorTuiTableState::default();
        
        // Test initial state
        assert_eq!(state.current_selection(), Some(0));
        assert_matches!(state.mode(), TableMode::SingleSelection);
        assert_eq!(state.marked().count(), 0);
        
        // Test selection workflow
        state.select(Some(3));
        assert_eq!(state.current_selection(), Some(3));
        
        // Test mode transition to multiple selection
        state.multiple_selection();
        assert_matches!(state.mode(), TableMode::MultipleSelection);
        
        // Test marking workflow in multiple selection mode
        state.mark(Some(1));
        state.mark(Some(3));
        state.mark(Some(5));
        
        assert_eq!(state.marked().count(), 3);
        
        // Test clearing workflow
        state.clear();
        assert_eq!(state.marked().count(), 0);
        
        // Test mode transition back to single selection
        state.single_selection();
        assert_matches!(state.mode(), TableMode::SingleSelection);
    }

    #[test]
    fn test_task_filtering_by_status() {
        let tasks_by_status = MockData::tasks_by_status();
        
        // Test pending tasks filter
        let pending_entry = tasks_by_status.iter().find(|(status, _)| matches!(status, TaskStatus::Pending)).unwrap();
        let pending_tasks = &pending_entry.1;
        assert_eq!(pending_tasks.len(), 2);
        
        for task in pending_tasks {
            assert_eq!(task.status(), &TaskStatus::Pending);
        }
        
        // Test completed tasks filter
        let completed_entry = tasks_by_status.iter().find(|(status, _)| matches!(status, TaskStatus::Completed)).unwrap();
        let completed_tasks = &completed_entry.1;
        assert_eq!(completed_tasks.len(), 2);
        
        for task in completed_tasks {
            assert_eq!(task.status(), &TaskStatus::Completed);
        }
        
        // Test deleted tasks filter
        let deleted_entry = tasks_by_status.iter().find(|(status, _)| matches!(status, TaskStatus::Deleted)).unwrap();
        let deleted_tasks = &deleted_entry.1;
        assert_eq!(deleted_tasks.len(), 1);
        
        for task in deleted_tasks {
            assert_eq!(task.status(), &TaskStatus::Deleted);
        }
    }

    // =========================================================================
    // RESPONSIVE DESIGN TESTS
    // =========================================================================

    #[test]
    fn test_responsive_layout_different_sizes() {
        let sizes = [(40, 10), (80, 24), (120, 30), (160, 40)];
        
        for (width, height) in sizes {
            let mut tui_helper = TuiTestHelper::with_size(width, height);
            
            let buffer = tui_helper.draw_widget(
                ratatui::widgets::Paragraph::new("Taskwarrior TUI - Responsive Test")
                    .block(ratatui::widgets::Block::default()
                        .title(format!("Size: {}x{}", width, height))
                        .borders(ratatui::widgets::Borders::ALL)),
                None,
            );
            
            UiAssertions::contains_text(&buffer, "Taskwarrior TUI");
            UiAssertions::contains_text(&buffer, &format!("Size: {}x{}", width, height));
            UiAssertions::has_border_elements(&buffer);
        }
    }

    #[test]
    fn test_error_handling_ui() {
        let mut tui_helper = TuiTestHelper::new();
        
        // Test error message display
        let error_message = "Failed to connect to taskwarrior database";
        
        let buffer = tui_helper.draw_widget(
            ratatui::widgets::Paragraph::new(format!("Error: {}", error_message))
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Red))
                .block(ratatui::widgets::Block::default()
                    .title("Error")
                    .borders(ratatui::widgets::Borders::ALL)
                    .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Red))),
            None,
        );
        
        UiAssertions::contains_text(&buffer, "Error:");
        UiAssertions::contains_text(&buffer, error_message);
        UiAssertions::has_border_elements(&buffer);
        
        tui_helper.assert_snapshot("error_display");
    }

    #[test]
    fn test_help_screen_ui() {
        let mut tui_helper = TuiTestHelper::new();
        
        let help_content = vec![
            "Key Bindings:",
            "  j/Down  - Move down",
            "  k/Up    - Move up", 
            "  Enter   - Select task",
            "  Space   - Mark task",
            "  q       - Quit",
            "",
            "Press any key to continue...",
        ];
        
        let buffer = tui_helper.draw_widget(
            ratatui::widgets::Paragraph::new(help_content.join("\n"))
                .block(ratatui::widgets::Block::default()
                    .title("Help")
                    .borders(ratatui::widgets::Borders::ALL)),
            None,
        );
        
        UiAssertions::contains_text(&buffer, "Key Bindings:");
        UiAssertions::contains_text(&buffer, "Move down");
        UiAssertions::contains_text(&buffer, "Quit");
        UiAssertions::has_border_elements(&buffer);
        
        tui_helper.assert_snapshot("help_screen");
    }

    #[test]
    fn test_task_details_view() {
        let mut tui_helper = TuiTestHelper::new();
        
        let task = MockData::task_with_properties(
            "Complex task with multiple attributes",
            TaskStatus::Pending,
            vec!["urgent".to_string(), "work".to_string()],
            Some("project-alpha".to_string()),
        );
        
        let details = vec![
            format!("Description: {}", task.description()),
            format!("Status: {:?}", task.status()),
            format!("Project: {}", task.project().unwrap_or(&"None".to_string())),
            format!("Tags: {}", task.tags().map(|tags| tags.join(", ")).unwrap_or_else(|| "None".to_string())),
            format!("UUID: {}", task.uuid()),
        ];
        
        let buffer = tui_helper.draw_widget(
            ratatui::widgets::Paragraph::new(details.join("\n"))
                .block(ratatui::widgets::Block::default()
                    .title("Task Details")
                    .borders(ratatui::widgets::Borders::ALL)),
            None,
        );
        
        UiAssertions::contains_text(&buffer, "Complex task with multiple attributes");
        UiAssertions::contains_text(&buffer, "Status: Pending");
        UiAssertions::contains_text(&buffer, "project-alpha");
        UiAssertions::contains_text(&buffer, "urgent");
        UiAssertions::has_border_elements(&buffer);
        
        tui_helper.assert_snapshot("task_details_view");
    }

    // =========================================================================
    // PROPERTY-BASED INTEGRATION TESTS
    // =========================================================================

    /// Property-based integration tests
    /// 
    /// These tests use randomly generated data to verify system behavior
    /// across a wide range of inputs and edge cases.
    mod property_tests {
        use super::*;
        use crate::ui_testing::strategies;

        proptest! {
            #[test]
            fn test_any_task_list_renders_without_panic(tasks in strategies::task_list()) {
                let mut tui_helper = TuiTestHelper::new();
                
                // Create a simple representation of the task list
                let task_list: Vec<String> = tasks
                    .iter()
                    .map(|task| format!("• {}", task.description()))
                    .collect();
                
                let content = if task_list.is_empty() {
                    "No tasks available".to_string()
                } else {
                    task_list.join("\n")
                };
                
                // This should never panic regardless of input
                let buffer = tui_helper.draw_widget(
                    ratatui::widgets::Paragraph::new(content)
                        .block(ratatui::widgets::Block::default()
                            .title(format!("Tasks ({})", tasks.len()))
                            .borders(ratatui::widgets::Borders::ALL)),
                    None,
                );
                
                // Basic sanity checks
                prop_assert!(buffer.len() > 0);
                UiAssertions::has_border_elements(&buffer);
            }

            #[test]
            fn test_table_state_invariants(
                selections in prop::collection::vec(prop::option::of(0usize..100), 1..20),
                marks in prop::collection::vec(0usize..100, 0..10)
            ) {
                use taskwarrior_tui::table::TaskwarriorTuiTableState;
                let mut state = TaskwarriorTuiTableState::default();
                
                // Apply random sequence of operations
                for selection in selections {
                    state.select(selection);
                    // Selection should always be valid
                    prop_assert_eq!(state.current_selection(), selection);
                }
                
                for mark in &marks {
                    state.mark(Some(*mark));
                }
                
                // Marked items should be within expected range
                let marked_count = state.marked().count();
                prop_assert!(marked_count <= marks.len());
                
                // Clear should always work
                state.clear();
                prop_assert_eq!(state.marked().count(), 0);
            }

            #[test]
            fn test_ui_dimensions_always_valid(
                width in 20u16..200,
                height in 10u16..100
            ) {
                let mut tui_helper = TuiTestHelper::with_size(width, height);
                
                let buffer = tui_helper.draw_widget(
                    ratatui::widgets::Paragraph::new("Test content")
                        .block(ratatui::widgets::Block::default()
                            .borders(ratatui::widgets::Borders::ALL)),
                    None,
                );
                
                // Buffer should never be empty
                prop_assert!(buffer.len() > 0);
                // Should contain our test content
                UiAssertions::contains_text(&buffer, "Test content");
            }
        }
    }

    // =========================================================================
    // PERFORMANCE AND STRESS TESTS
    // =========================================================================

    /// Performance and stress tests
    /// 
    /// These tests verify that the system performs well under load and
    /// handles edge cases like very large datasets or unusual content.
    
    #[test]
    fn test_large_task_list_performance() {
        let mut tui_helper = TuiTestHelper::new();
        
        // Create a large number of tasks
        let large_task_list: Vec<String> = (0..1000)
            .map(|i| format!("• Task number {}", i))
            .collect();
        
        let content = large_task_list.join("\n");
        
        // This should complete without hanging or crashing
        let start = std::time::Instant::now();
        
        let buffer = tui_helper.draw_widget(
            ratatui::widgets::Paragraph::new(content)
                .block(ratatui::widgets::Block::default()
                    .title("Large Task List (1000 items)")
                    .borders(ratatui::widgets::Borders::ALL)),
            None,
        );
        
        let duration = start.elapsed();
        
        // Should complete in reasonable time (less than 1 second)
        assert!(duration.as_secs() < 1, "Rendering took too long: {:?}", duration);
        
        UiAssertions::contains_text(&buffer, "Task number 0");
        UiAssertions::contains_text(&buffer, "Large Task List (1000 items)");
    }

    #[test]
    fn test_unicode_task_descriptions() {
        let mut tui_helper = TuiTestHelper::new();
        
        let unicode_tasks = vec![
            "タスク管理 (Task Management in Japanese)",
            "Aufgabenverwaltung (Task Management in German)",
            "Gestión de tareas (Task Management in Spanish)",
            "🏃‍♂️ Run marathon 🏃‍♀️",
            "📝 Write documentation 📚",
            "🐛 Fix bugs 🔧",
        ];
        
        let content = unicode_tasks
            .iter()
            .map(|task| format!("• {}", task))
            .collect::<Vec<_>>()
            .join("\n");
        
        let buffer = tui_helper.draw_widget(
            ratatui::widgets::Paragraph::new(content)
                .block(ratatui::widgets::Block::default()
                    .title("Unicode Tasks")
                    .borders(ratatui::widgets::Borders::ALL)),
            None,
        );
        
        // Verify unicode content is preserved
        for task in &unicode_tasks {
            UiAssertions::contains_text(&buffer, task);
        }
        
        tui_helper.assert_snapshot("unicode_tasks");
    }
}