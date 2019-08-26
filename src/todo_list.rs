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
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.todos
                    .iter()
                    .enumerate()
                    .map(|(pos, todo)| {
                        if pos == self.selected_index {
                            format!("{}{}{}", Fg(Black), Bg(White), todo)
                        } else {
                            format!("{}{}{}", Fg(Reset), Bg(Reset), todo)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n\r");
        write!(f, "{}", s)
    }
}
