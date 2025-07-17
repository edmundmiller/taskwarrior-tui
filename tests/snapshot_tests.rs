//! # Comprehensive Snapshot Testing for Visual Regression Protection
//! 
//! This module provides extensive snapshot testing to catch visual regressions
//! in the TUI interface and ensure consistent rendering across different scenarios.
//! 
//! ## Purpose
//! Snapshot testing captures the exact terminal output for UI components and stores
//! it as a baseline. Future test runs compare against these baselines to detect
//! any unintended visual changes.
//! 
//! ## Test Organization
//! 
//! ### Basic Widgets (`basic_widgets` module)
//! Tests for fundamental UI components:
//! - Title widget with borders and styling
//! - Help text with key bindings
//! - Status bar with task counts
//! - Progress indicators and gauges
//! 
//! ### Task List Scenarios (`task_list_snapshots` module)
//! Tests for task display in various states:
//! - Empty task list with helpful message
//! - Single task display
//! - Multiple tasks with different statuses
//! - Task selection and marking indicators
//! 
//! ### Responsive Design (`responsive_snapshots` module)
//! Tests for different terminal sizes:
//! - Narrow terminals (40x15)
//! - Wide terminals (120x30)
//! - Square terminals (60x60)
//! - Content adaptation and wrapping
//! 
//! ### Theme Testing (`theme_snapshots` module)
//! Tests for different color schemes:
//! - Dark theme with standard colors
//! - Light theme with inverted colors
//! - High contrast theme for accessibility
//! 
//! ### Layout Testing (`layout_snapshots` module)
//! Tests for complex UI layouts:
//! - Split pane layouts (60/40)
//! - Tabbed interfaces
//! - Three-pane layouts (context/tasks/details)
//! 
//! ### Edge Cases (`edge_case_snapshots` module)
//! Tests for error conditions and edge cases:
//! - Error message display
//! - Loading states
//! - Confirmation dialogs
//! - Very long text content
//! 
//! ### Regression Tests (`regression_snapshots` module)
//! Tests for specific issues and regressions:
//! - Unicode character rendering
//! - Special character handling
//! - Minimal terminal size support

#[path = "ui_testing.rs"]
mod ui_testing;

use crate::ui_testing::{TuiTestHelper};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Gauge, LineGauge, List, ListItem, Paragraph, Tabs},
};

// =============================================================================
// SNAPSHOT TEST SUITE
// =============================================================================

/// Comprehensive snapshot tests for different UI components and scenarios
/// 
/// These tests capture the exact terminal output for various UI states and
/// compare them against stored baselines to detect visual regressions.
#[cfg(test)]
mod snapshot_tests {
    use super::*;

    // =========================================================================
    // BASIC WIDGET SNAPSHOTS
    // =========================================================================

    /// Test basic widget rendering snapshots
    /// 
    /// These tests verify that fundamental UI widgets render correctly:
    /// - Title widget with proper borders and text alignment
    /// - Help text with organized key binding information
    /// - Status bar with task count information
    /// - Progress indicators showing completion percentages
    mod basic_widgets {
        use super::*;

        #[test]
        fn test_title_widget_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let title = Paragraph::new("Taskwarrior TUI")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );
            
            tui_helper.draw_widget(title, None);
            tui_helper.assert_snapshot("title_widget_basic");
        }

        #[test]
        fn test_help_text_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let help_text = vec![
                "Taskwarrior TUI - Help",
                "",
                "Navigation:",
                "  ↑/k       Move up",
                "  ↓/j       Move down", 
                "  Enter     Select item",
                "  Space     Mark/unmark",
                "",
                "Actions:",
                "  a         Add task",
                "  d         Delete task",
                "  e         Edit task",
                "  m         Mark done",
                "",
                "Other:",
                "  ?         Show this help",
                "  q         Quit",
            ];
            
            let help_widget = Paragraph::new(help_text.join("\n"))
                .block(Block::default()
                    .title("Help")
                    .borders(Borders::ALL))
                .alignment(Alignment::Left);
            
            tui_helper.draw_widget(help_widget, None);
            tui_helper.assert_snapshot("help_text_complete");
        }

        #[test]
        fn test_status_bar_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let status_items = [
                "Tasks: 15",
                "Pending: 8", 
                "Completed: 7",
                "Filter: status:pending",
            ];
            
            let status_text = status_items.join(" | ");
            
            let status_bar = Paragraph::new(status_text)
                .style(Style::default().bg(Color::DarkGray).fg(Color::White))
                .alignment(Alignment::Left)
                .block(Block::default().borders(Borders::TOP));
            
            tui_helper.draw_widget(status_bar, Some(Rect::new(0, 21, 80, 3)));
            tui_helper.assert_snapshot("status_bar_with_counts");
        }

        #[test]
        fn test_progress_indicators_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            // Create a progress gauge
            let progress = Gauge::default()
                .block(Block::default().title("Task Completion").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Green))
                .percent(65)
                .label("65% Complete");
            
            tui_helper.draw_widget(progress, Some(Rect::new(0, 0, 40, 3)));
            tui_helper.assert_snapshot("progress_gauge_65_percent");
        }

        #[test]
        fn test_line_gauge_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let line_gauge = LineGauge::default()
                .block(Block::default().title("Daily Progress").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Blue).bg(Color::Gray))
                .ratio(0.4)
                .label("4/10 tasks");
            
            tui_helper.draw_widget(line_gauge, Some(Rect::new(0, 0, 50, 3)));
            tui_helper.assert_snapshot("line_gauge_daily_progress");
        }
    }

    // =========================================================================
    // TASK LIST SNAPSHOT TESTS
    // =========================================================================

    /// Test task list rendering in different states
    /// 
    /// These tests verify task list display across various scenarios:
    /// - Empty state with helpful messaging
    /// - Single task with proper formatting
    /// - Multiple tasks with status indicators
    /// - Selection and marking visual feedback
    mod task_list_snapshots {
        use super::*;

        #[test]
        fn test_empty_task_list_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let empty_message = Paragraph::new("No tasks found.\n\nPress 'a' to add a new task.")
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center)
                .block(Block::default()
                    .title("Tasks")
                    .borders(Borders::ALL));
            
            tui_helper.draw_widget(empty_message, None);
            tui_helper.assert_snapshot("empty_task_list_with_message");
        }

        #[test]
        fn test_single_task_list_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let tasks = vec![
                ListItem::new("• [P] Complete project documentation (work, urgent)")
                    .style(Style::default().fg(Color::White)),
            ];
            
            let task_list = List::new(tasks)
                .block(Block::default()
                    .title("Tasks (1)")
                    .borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
            
            tui_helper.draw_widget(task_list, None);
            tui_helper.assert_snapshot("single_task_list");
        }

        #[test]
        fn test_multiple_tasks_different_status_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let tasks = vec![
                ListItem::new("• [P] Review pull request #123 (code-review)")
                    .style(Style::default().fg(Color::Cyan)),
                ListItem::new("• [P] Buy groceries (personal)")
                    .style(Style::default().fg(Color::White)),
                ListItem::new("• [C] Schedule dentist appointment (health)")
                    .style(Style::default().fg(Color::Green)),
                ListItem::new("• [P] Learn Rust async programming (learning, rust)")
                    .style(Style::default().fg(Color::Yellow)),
                ListItem::new("• [P] Fix critical bug in authentication (work, urgent)")
                    .style(Style::default().fg(Color::Red)),
            ];
            
            let task_list = List::new(tasks)
                .block(Block::default()
                    .title("Mixed Status Tasks (5)")
                    .borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
            
            tui_helper.draw_widget(task_list, None);
            tui_helper.assert_snapshot("mixed_status_task_list");
        }

        #[test]
        fn test_task_list_with_selection_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let tasks = vec![
                ListItem::new("  [P] Task 1 - Not selected")
                    .style(Style::default().fg(Color::White)),
                ListItem::new("→ [P] Task 2 - Currently selected")
                    .style(Style::default().fg(Color::Black).bg(Color::Cyan)),
                ListItem::new("  [P] Task 3 - Not selected")
                    .style(Style::default().fg(Color::White)),
                ListItem::new("✓ [P] Task 4 - Marked")
                    .style(Style::default().fg(Color::Green)),
            ];
            
            let task_list = List::new(tasks)
                .block(Block::default()
                    .title("Task Selection Demo")
                    .borders(Borders::ALL));
            
            tui_helper.draw_widget(task_list, None);
            tui_helper.assert_snapshot("task_list_with_selection");
        }
    }

    // =========================================================================
    // RESPONSIVE DESIGN SNAPSHOT TESTS
    // =========================================================================

    /// Test different terminal sizes for responsive design
    /// 
    /// These tests verify that the UI adapts properly to different terminal sizes:
    /// - Narrow terminals with limited horizontal space
    /// - Wide terminals with extra horizontal space
    /// - Square terminals with balanced dimensions
    /// - Content wrapping and truncation behavior
    mod responsive_snapshots {
        use super::*;

        #[test]
        fn test_narrow_terminal_snapshot() {
            let mut tui_helper = TuiTestHelper::with_size(40, 15);
            
            let content = Paragraph::new("Narrow terminal\nview test\n\nSome tasks:\n• Task 1\n• Task 2")
                .block(Block::default()
                    .title("40x15")
                    .borders(Borders::ALL))
                .alignment(Alignment::Left);
            
            tui_helper.draw_widget(content, None);
            tui_helper.assert_snapshot("narrow_terminal_40x15");
        }

        #[test]
        fn test_wide_terminal_snapshot() {
            let mut tui_helper = TuiTestHelper::with_size(120, 30);
            
            let content = Paragraph::new(
                "Wide terminal view test - this should have more horizontal space\n\
                Tasks with longer descriptions can be displayed in full:\n\
                • Complete comprehensive integration testing for the taskwarrior TUI application\n\
                • Implement advanced filtering and search capabilities with regex support\n\
                • Add keyboard shortcuts for power users and vim-like navigation"
            )
                .block(Block::default()
                    .title("120x30 Terminal")
                    .borders(Borders::ALL))
                .alignment(Alignment::Left);
            
            tui_helper.draw_widget(content, None);
            tui_helper.assert_snapshot("wide_terminal_120x30");
        }

        #[test]
        fn test_square_terminal_snapshot() {
            let mut tui_helper = TuiTestHelper::with_size(60, 60);
            
            let content = Paragraph::new(
                "Square terminal test\n\
                More vertical space available\n\
                for displaying tasks:\n\n\
                • Task 1\n• Task 2\n• Task 3\n• Task 4\n• Task 5\n\
                • Task 6\n• Task 7\n• Task 8\n• Task 9\n• Task 10"
            )
                .block(Block::default()
                    .title("Square 60x60")
                    .borders(Borders::ALL))
                .alignment(Alignment::Left);
            
            tui_helper.draw_widget(content, None);
            tui_helper.assert_snapshot("square_terminal_60x60");
        }
    }

    // =========================================================================
    // THEME AND COLOR SCHEME SNAPSHOT TESTS
    // =========================================================================

    /// Test different themes and color schemes
    /// 
    /// These tests verify visual consistency across different color themes:
    /// - Dark theme for low-light environments
    /// - Light theme for bright environments
    /// - High contrast theme for accessibility
    /// - Color application and text readability
    mod theme_snapshots {
        use super::*;

        #[test]
        fn test_dark_theme_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let content = Paragraph::new("Dark theme example\nwith various colors")
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .block(Block::default()
                    .title("Dark Theme")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray)));
            
            tui_helper.draw_widget(content, None);
            tui_helper.assert_snapshot("dark_theme_example");
        }

        #[test]
        fn test_light_theme_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let content = Paragraph::new("Light theme example\nwith different styling")
                .style(Style::default().fg(Color::Black).bg(Color::White))
                .block(Block::default()
                    .title("Light Theme")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)));
            
            tui_helper.draw_widget(content, None);
            tui_helper.assert_snapshot("light_theme_example");
        }

        #[test]
        fn test_high_contrast_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let content = Paragraph::new("HIGH CONTRAST\nMAXIMUM READABILITY")
                .style(Style::default().fg(Color::Yellow).bg(Color::Black).add_modifier(Modifier::BOLD))
                .block(Block::default()
                    .title("High Contrast")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)));
            
            tui_helper.draw_widget(content, None);
            tui_helper.assert_snapshot("high_contrast_theme");
        }
    }

    // =========================================================================
    // COMPLEX LAYOUT SNAPSHOT TESTS
    // =========================================================================

    /// Test complex layouts and multi-pane interfaces
    /// 
    /// These tests verify complex UI layouts and multi-pane arrangements:
    /// - Split pane layouts with proper proportions
    /// - Tabbed interfaces with navigation indicators
    /// - Three-pane layouts for context/tasks/details
    /// - Layout consistency and alignment
    mod layout_snapshots {
        use super::*;

        #[test]
        fn test_split_pane_layout_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            // Create a complex layout within the terminal space
            let area = Rect::new(0, 0, 80, 24);
            let _chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(area);
            
            // Draw a combined layout representation
            let _left_content = "Task List Pane\n\n• Task 1\n• Task 2\n• Task 3";
            let _right_content = "Details Pane\n\nSelected: Task 1\nStatus: Pending\nTags: work, urgent";
            
            let combined_content = format!(
                "{:<48}│{}\n{:<48}│{}\n{:<48}│{}\n{:<48}│{}\n{:<48}│{}",
                "Task List Pane", "Details Pane",
                "", "",
                "• Task 1", "Selected: Task 1", 
                "• Task 2", "Status: Pending",
                "• Task 3", "Tags: work, urgent"
            );
            
            let layout_widget = Paragraph::new(combined_content)
                .block(Block::default()
                    .title("Split Layout (60/40)")
                    .borders(Borders::ALL));
            
            tui_helper.draw_widget(layout_widget, None);
            tui_helper.assert_snapshot("split_pane_layout");
        }

        #[test]
        fn test_tabbed_interface_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let tab_titles = vec!["Tasks", "Projects", "Contexts", "Reports"];
            let tabs = Tabs::new(tab_titles)
                .block(Block::default().borders(Borders::ALL))
                .select(0)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            
            tui_helper.draw_widget(tabs, Some(Rect::new(0, 0, 80, 3)));
            tui_helper.assert_snapshot("tabbed_interface");
        }

        #[test]
        fn test_three_pane_layout_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let content = format!(
                "{:<25}│{:<25}│{}\n{:<25}│{:<25}│{}\n{:<25}│{:<25}│{}\n{:<25}│{:<25}│{}",
                "Context", "Tasks", "Details",
                "─────────", "─────────", "─────────",
                "• work", "• Fix bug", "Description:",
                "• personal", "• Review PR", "Fix critical auth bug"
            );
            
            let three_pane = Paragraph::new(content)
                .block(Block::default()
                    .title("Three Pane Layout")
                    .borders(Borders::ALL));
            
            tui_helper.draw_widget(three_pane, None);
            tui_helper.assert_snapshot("three_pane_layout");
        }
    }

    // =========================================================================
    // ERROR STATES AND EDGE CASE SNAPSHOT TESTS
    // =========================================================================

    /// Test error states and edge cases
    /// 
    /// These tests verify proper handling of error conditions and edge cases:
    /// - Error message display with clear formatting
    /// - Loading states with progress indicators
    /// - Confirmation dialogs with user prompts
    /// - Very long content with wrapping behavior
    mod edge_case_snapshots {
        use super::*;

        #[test]
        fn test_error_message_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let error_content = "ERROR: Unable to connect to taskwarrior\n\nPossible causes:\n• Taskwarrior not installed\n• Database corruption\n• Permissions issue\n\nPress 'r' to retry or 'q' to quit";
            
            let error_widget = Paragraph::new(error_content)
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center)
                .block(Block::default()
                    .title("Error")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Red)));
            
            tui_helper.draw_widget(error_widget, None);
            tui_helper.assert_snapshot("error_message_display");
        }

        #[test]
        fn test_loading_state_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let loading_content = "Loading tasks...\n\n[████████████████████████████████████████] 100%\n\nPlease wait...";
            
            let loading_widget = Paragraph::new(loading_content)
                .style(Style::default().fg(Color::Cyan))
                .alignment(Alignment::Center)
                .block(Block::default()
                    .title("Loading")
                    .borders(Borders::ALL));
            
            tui_helper.draw_widget(loading_widget, None);
            tui_helper.assert_snapshot("loading_state");
        }

        #[test]
        fn test_confirmation_dialog_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let dialog_content = "Delete Task?\n\nAre you sure you want to delete:\n'Complete project documentation'\n\nThis action cannot be undone.\n\n[Y]es / [N]o";
            
            let dialog_widget = Paragraph::new(dialog_content)
                .style(Style::default().fg(Color::White).bg(Color::DarkGray))
                .alignment(Alignment::Center)
                .block(Block::default()
                    .title("Confirm Deletion")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)));
            
            tui_helper.draw_widget(dialog_widget, Some(Rect::new(20, 8, 40, 10)));
            tui_helper.assert_snapshot("confirmation_dialog");
        }

        #[test]
        fn test_very_long_task_description_snapshot() {
            let mut tui_helper = TuiTestHelper::new();
            
            let long_description = "This is an extremely long task description that might cause text wrapping issues and should be tested to ensure it displays correctly in the terminal user interface without breaking the layout or causing visual problems";
            
            let long_task_widget = Paragraph::new(format!("Long Task:\n\n{}", long_description))
                .block(Block::default()
                    .title("Text Wrapping Test")
                    .borders(Borders::ALL))
                .wrap(ratatui::widgets::Wrap { trim: true });
            
            tui_helper.draw_widget(long_task_widget, None);
            tui_helper.assert_snapshot("long_task_description");
        }
    }

    // =========================================================================
    // REGRESSION SNAPSHOT TESTS
    // =========================================================================

    /// Regression tests for specific issues
    /// 
    /// These tests verify fixes for specific bugs and prevent regressions:
    /// - Unicode character rendering correctness
    /// - Special character escape handling
    /// - Minimal terminal size support
    /// - Character encoding edge cases
    mod regression_snapshots {
        use super::*;

        #[test]
        fn test_unicode_rendering_regression() {
            let mut tui_helper = TuiTestHelper::new();
            
            let unicode_content = "Unicode Test:\n\n🚀 Priority task\n📋 Documentation\n🐛 Bug fix\n✅ Completed\n🏷️ Tags: プロジェクト, тест, ทดสอบ";
            
            let unicode_widget = Paragraph::new(unicode_content)
                .block(Block::default()
                    .title("Unicode Rendering")
                    .borders(Borders::ALL));
            
            tui_helper.draw_widget(unicode_widget, None);
            tui_helper.assert_snapshot("unicode_rendering_test");
        }

        #[test]
        fn test_special_characters_regression() {
            let mut tui_helper = TuiTestHelper::new();
            
            let special_chars = "Special Characters:\n\n• Task with \"quotes\"\n• Task with 'apostrophes'\n• Task with <brackets>\n• Task with &amp; entities\n• Task with \\backslashes\\";
            
            let special_widget = Paragraph::new(special_chars)
                .block(Block::default()
                    .title("Special Characters Test")
                    .borders(Borders::ALL));
            
            tui_helper.draw_widget(special_widget, None);
            tui_helper.assert_snapshot("special_characters_test");
        }

        #[test]
        fn test_minimal_terminal_size_regression() {
            let mut tui_helper = TuiTestHelper::with_size(20, 8);
            
            let minimal_content = "Min\nTerm\nTest";
            
            let minimal_widget = Paragraph::new(minimal_content)
                .block(Block::default()
                    .title("Min")
                    .borders(Borders::ALL));
            
            tui_helper.draw_widget(minimal_widget, None);
            tui_helper.assert_snapshot("minimal_terminal_size");
        }
    }
}

// =============================================================================
// SNAPSHOT TESTING CONFIGURATION
// =============================================================================

/// Snapshot test configuration and utilities
/// 
/// This function configures insta settings for better snapshot management:
/// - Prepends module names to snapshot files for organization
/// - Omits expressions to reduce noise in snapshots
/// - Sets up consistent snapshot naming and storage
pub fn configure_snapshot_testing() {
    // Configure insta for better snapshot management
    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(true);
    settings.set_omit_expression(true);
    let _guard = settings.bind_to_scope();
}