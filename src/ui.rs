use ratatui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  symbols,
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, Cell, LineGauge, Paragraph, Row, Table},
  Frame,
};

use crate::app::TaskwarriorTui;

pub fn draw(rect: &mut Frame, app: &TaskwarriorTui) {
  let size = rect.size();
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)].as_ref())
    .split(size);

  let title = draw_title();
  rect.render_widget(title, chunks[0]);
}

fn draw_title<'a>() -> Paragraph<'a> {
  Paragraph::new("Taskwarrior TUI")
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Center)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Plain),
    )
}

#[cfg(test)]
mod tests {
  use super::*;
  use insta::assert_snapshot;
  use ratatui::{backend::TestBackend, Terminal};

  #[test]
  fn test_draw_title_widget() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    
    terminal
      .draw(|frame| {
        let title = draw_title();
        frame.render_widget(title, frame.size());
      })
      .unwrap();

    let buffer = terminal.backend().buffer();
    let buffer_content: String = buffer
      .content
      .iter()
      .map(|cell| cell.symbol())
      .collect();
    assert_snapshot!("title_widget", buffer_content);
  }

  #[test]
  fn test_draw_title_contains_expected_text() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    
    terminal
      .draw(|frame| {
        let title = draw_title();
        frame.render_widget(title, frame.size());
      })
      .unwrap();

    // Convert buffer to string by iterating over cells
    let buffer = terminal.backend().buffer();
    let buffer_content: String = buffer
      .content
      .iter()
      .map(|cell| cell.symbol().to_string())
      .collect();
    
    assert!(buffer_content.contains("Taskwarrior TUI"));
  }

  #[test]
  fn test_draw_title_has_borders() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    
    terminal
      .draw(|frame| {
        let title = draw_title();
        frame.render_widget(title, frame.size());
      })
      .unwrap();

    // Convert buffer to string by iterating over cells
    let buffer = terminal.backend().buffer();
    let buffer_content: String = buffer
      .content
      .iter()
      .map(|cell| cell.symbol().to_string())
      .collect();
    
    // Check for border characters
    assert!(buffer_content.contains("┌") || buffer_content.contains("┐") || 
            buffer_content.contains("└") || buffer_content.contains("┘"));
  }

  #[test]
  fn test_draw_title_different_sizes() {
    // Test with different terminal sizes
    let sizes = [(40, 10), (80, 24), (120, 30)];
    
    for (width, height) in sizes {
      let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
      
      terminal
        .draw(|frame| {
          let title = draw_title();
          frame.render_widget(title, frame.size());
        })
        .unwrap();

      // Convert buffer to string by iterating over cells
      let buffer = terminal.backend().buffer();
      let buffer_content: String = buffer
        .content
        .iter()
        .map(|cell| cell.symbol().to_string())
        .collect();
      
      assert!(buffer_content.contains("Taskwarrior TUI"));
    }
  }
}
