extern crate termion;

use std::fmt;
use termion::color::*;
use crate::todo::Todo;

pub struct TodoList<'a> {
    todos: &'a Vec<Todo>,
    selected_index: usize,
}

impl<'a> TodoList<'a> {
    pub fn new() -> TodoList<'a> {
        TodoList {
            todos: &mut Vec::new(),
            selected_index: 0,
        }
    }

    pub fn add(&self, todo: Todo) {
        self.todos.push(todo);
    }
}

impl<'a> fmt::Display for TodoList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: Vec<String> = self.todos
                    .iter()
                    .enumerate()
                    .map(|(pos, todo)| {
                        if pos == self.selected_index {
                            format!("{}{}{}", Fg(Black), Bg(White), todo)
                        } else {
                            format!("{}", todo)
                        }
                    }).collect();
        write!(f, "{}", s.concat())
    }
}
