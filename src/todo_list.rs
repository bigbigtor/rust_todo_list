extern crate termion;

use std::fmt;
use termion::color::*;
use crate::todo::Todo;

pub struct TodoList {
    todos: Vec<Todo>,
    selected_index: usize,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
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
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.todos
                    .iter()
                    .enumerate()
                    .map(|(pos, todo)| {
                        if pos == self.selected_index {
                            format!("{}{}{}{}{}", Fg(Black), Bg(White), todo, Fg(Reset), Bg(Reset))
                        } else {
                            format!("{}", todo)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\r\n");
        write!(f, "{}", s)
    }
}
