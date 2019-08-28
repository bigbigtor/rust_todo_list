extern crate dirs;

use std::fs;
use std::io;
use termion::event::Key;
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

    fn toggle_selected_todo(&mut self) {
        let todo = self.todos[self.selected_index].toggle_complete();
        self.todos[self.selected_index] = todo;
    }

    fn select_previous_todo(&mut self) {
        if self.selected_index < (self.todos.len() - 1) {
            self.selected_index += 1;
        }
    }

    fn select_next_todo(&mut self) {
        if 0 < self.selected_index {
            self.selected_index -= 1;
        }
    }

    pub fn to_string_list(&self) -> Vec<String> {
        self.todos.iter()
                  .map(|t| format!("{}", t))
                  .collect::<Vec<String>>()
    }

    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut todo_file_path = dirs::config_dir().unwrap();
        todo_file_path.push(".todo_list");
        let content = fs::read_to_string(todo_file_path)?;
        content.lines()
               .map(|line| Todo::build_from_storage(line))
               .for_each(|todo| self.add(todo));
        Ok(())
    }

    pub fn persist (&self) -> io::Result<()> {
        let content = self.todos
                          .iter()
                          .map(|t| t.to_storage_line())
                          .collect::<Vec<String>>()
                          .join("\r\n");
        let mut todo_file_path = dirs::config_dir().unwrap();
        todo_file_path.push(".todo_list");
        fs::write(todo_file_path, content)?;
        Ok(())
    }

    pub fn handle_event(&mut self, key: Key) {
        match key {
            Key::Char('j') => self.select_previous_todo(),
            Key::Char('k') => self.select_next_todo(),
            Key::Char('t') => self.toggle_selected_todo(),
            _ => {}
        }
    }

}
