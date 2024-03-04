use gtk::{prelude::*, Label};
use gtk::{Box as GtkBox, Orientation};

use crate::models::Todo;

use super::View;

pub struct TodoGuiView;

impl View<Todo, GtkBox> for TodoGuiView {
  fn render(todo: &Todo) -> GtkBox {
    let row = GtkBox::new(Orientation::Horizontal, 0);
    let label = Label::new(None);
    label.set_text(&format!("{} - {}{}", todo.id, todo.title, if todo.completed { " (Completed)" } else { "" }));
    row.append(&label);

    row
  }
}
