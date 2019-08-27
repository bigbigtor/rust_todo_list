use crate::todo::Todo;

pub struct AppState {
    todos: Vec<Todo>,
    selected_index: usize,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            todos: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn toggle_selected_todo(&mut self) {
        let todo = self.todos[self.selected_index].toggle_complete();
        self.todos[self.selected_index] = todo;
    }

    pub fn select_previous_todo(&mut self) {
        if self.selected_index < (self.todos.len() - 1) {
            self.selected_index += 1;
        }
    }

    pub fn select_next_todo(&mut self) {
        if 0 < self.selected_index {
            self.selected_index -= 1;
        }
    }

    pub fn to_storage_content(&self) -> String {
        self.todos.iter()
                  .map(|todo| todo.to_storage_line())
                  .collect::<Vec<String>>()
                  .join("\r\n")
    }
}
