use rustyline::line_buffer::{ChangeListener, DeleteListener, Direction};

/// Undo manager
#[derive(Default)]
pub struct Changeset {}

impl DeleteListener for Changeset {
  fn delete(&mut self, _idx: usize, _string: &str, _: Direction) {}
}

impl ChangeListener for Changeset {
  fn insert_char(&mut self, _idx: usize, _c: char) {}

  fn insert_str(&mut self, _idx: usize, _string: &str) {}

  fn replace(&mut self, _idx: usize, _old: &str, _new: &str) {}
}
