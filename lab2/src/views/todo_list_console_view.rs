use crate::models::Todo;
use super::{View, TodoConsoleView};

pub struct TodoListConsoleView;

impl View<Vec<Todo>> for TodoListConsoleView {
  fn render(todos: &Vec<Todo>) {
    for todo in todos {
      TodoConsoleView::render(todo);
    }
  }
}
