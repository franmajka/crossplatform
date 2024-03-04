pub trait View<T, R = ()> {
  fn render(data: &T) -> R;
}


mod todo_console_view;
pub use todo_console_view::TodoConsoleView;

mod todo_list_console_view;
pub use todo_list_console_view::TodoListConsoleView;

mod todo_list_gui_view;
pub use todo_list_gui_view::TodoListGuiView;

mod todo_gui_view;
pub use todo_gui_view::TodoGuiView;
