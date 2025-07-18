use ratatui::{
  buffer::Buffer,
  layout::{Alignment, Rect},
  style::{Modifier, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

const TEXT: &str = include_str!("../docs/keybindings.md");

pub struct Help {
  pub title: String,
  pub scroll: u16,
  pub text_height: usize,
}

impl Help {
  pub fn new() -> Self {
    Self {
      title: "Help".to_string(),
      scroll: 0,
      text_height: TEXT.lines().count(),
    }
  }
}

impl Default for Help {
  fn default() -> Self {
    Self::new()
  }
}

impl Widget for &Help {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let text: Vec<Line> = TEXT.lines().map(|line| Line::from(format!("{}\n", line))).collect();
    Paragraph::new(text)
      .block(
        Block::default()
          .title(Span::styled(&self.title, Style::default().add_modifier(Modifier::BOLD)))
          .borders(Borders::ALL)
          .border_type(BorderType::Rounded),
      )
      .alignment(Alignment::Left)
      .scroll((self.scroll, 0))
      .render(area, buf);
  }
}
