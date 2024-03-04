use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

use crate::controllers::TodoController;
use crate::views::{TodoListGuiView, View};

const APP_ID: &str = "org.example.mvc-todo";

pub fn run_app() -> glib::ExitCode {
  // Create a new application
  let app = Application::builder().application_id(APP_ID).build();

  // Connect to "activate" signal of `app`
  app.connect_activate(build_ui);

  // Run the application
  app.run()
}

fn build_ui(app: &Application) {
  // Create a window
  let window = ApplicationWindow::builder()
    .application(app)
    .title("TODO MVC Rust GTK4")
    .build();

  let todo_list = TodoListGuiView::render(&TodoController::get_todos());
  window.set_child(Some(&todo_list));

  // Present window
  window.present();
}
