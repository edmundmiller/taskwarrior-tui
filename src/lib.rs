//! Taskwarrior TUI Library
//! 
//! This library provides the core components for the taskwarrior-tui application.

pub mod app;
pub mod calendar;
pub mod completion;
pub mod config;
pub mod event;
pub mod help;
pub mod history;
pub mod keyconfig;
pub mod pane;
pub mod scrollbar;
pub mod table;
pub mod task_report;
pub mod ui;
pub mod utils;
pub mod backend;
pub mod action;
pub mod cli;
pub mod timewarrior;

// Re-export commonly used types
pub use app::TaskwarriorTui;
pub use config::Config;
pub use table::{TableMode, TaskwarriorTuiTableState};