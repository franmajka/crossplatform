use crate::models::Todo;

pub struct  TodoController;

impl TodoController {
  pub fn get_todos() -> Vec<Todo> {
    vec![
      Todo {
        id: 1,
        title: "Buy milk".to_string(),
        completed: false,
      },
      Todo {
        id: 2,
        title: "Buy eggs".to_string(),
        completed: true,
      },
      Todo {
        id: 3,
        title: "Buy bread".to_string(),
        completed: false,
      },
    ]
  }
}
