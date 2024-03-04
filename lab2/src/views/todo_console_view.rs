use crate::models::Todo;
use super::View;

pub struct TodoConsoleView;

impl View<Todo> for TodoConsoleView {
  fn render(data: &Todo) {
    println!("{} - {}{}", data.id, data.title, if data.completed { " (Completed)" } else { "" });
  }
}
