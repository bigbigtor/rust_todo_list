extern crate dirs;

use std::fs;
use std::io;
use termion::event::Key;
use crate::todo::Todo;

#[derive(PartialEq, Debug)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct AppState {
    pub todos: Vec<Todo>,
    pub selected_index: usize,
    pub mode: Mode,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            todos: Vec::new(),
            selected_index: 0,
            mode: Mode::Normal,
        }
    }

    pub fn add(&mut self, idx: usize) {
        let todo = Todo::new(String::new(), false);
        self.todos.insert(idx, todo);
    }

    fn toggle_selected_todo(&mut self) {
        self.todos[self.selected_index].toggle_complete();
    }

    fn delete_selected_todo(&mut self) {
        self.todos.remove(self.selected_index);
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

    pub fn load(&mut self) -> io::Result<()> {
        let mut todo_file_path = dirs::config_dir().unwrap();
        todo_file_path.push(".todo_list");
        let content = fs::read_to_string(todo_file_path)?;
        content.lines()
               .map(|line| Todo::build_from_storage(line))
               .for_each(|todo| self.todos.push(todo));
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
        match self.mode {
            Mode::Normal => {
                match key {
                    Key::Char('j') => self.select_previous_todo(),
                    Key::Char('k') => self.select_next_todo(),
                    Key::Char('t') => self.toggle_selected_todo(),
                    Key::Char('d') => self.delete_selected_todo(),
                    Key::Char('i') => {
                        self.add(self.selected_index);
                        self.mode = Mode::Insert;
                    },
                    Key::Char('a') => {
                        self.selected_index += 1;
                        self.add(self.selected_index);
                        self.mode = Mode::Insert;
                    },
                    _ => {}
                }
            },
            Mode::Insert => {
                match key {
                    Key::Esc => self.mode = Mode::Normal,
                    Key::Char(c) => {
                        self.todos[self.selected_index]
                            .description_mut()
                            .push(c);
                    },
                    Key::Backspace => {
                        self.todos[self.selected_index]
                            .description_mut()
                            .pop();
                    },
                    _ => {}
                }
            },
        }
    }

}
