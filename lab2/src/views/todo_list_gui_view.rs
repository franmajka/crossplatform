use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation};

use crate::models::Todo;

use super::{TodoGuiView, View};

pub struct TodoListGuiView;

impl View<Vec<Todo>, GtkBox> for TodoListGuiView {
  fn render(todos: &Vec<Todo>) -> GtkBox {
    let todo_list = GtkBox::new(Orientation::Vertical, 10);
    for todo in todos {
      let row = TodoGuiView::render(todo);
      todo_list.append(&row);
    }
    todo_list
  }
}
