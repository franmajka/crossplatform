use crate::{controllers::TodoController, views::{TodoListConsoleView, View}};

pub fn run_app() {
  TodoListConsoleView::render(&TodoController::get_todos());
}
