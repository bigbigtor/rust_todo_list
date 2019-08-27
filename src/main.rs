extern crate termion;
extern crate dirs;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::fs;

mod todo;
mod todo_list;

fn main() {
    let mut todo_list = todo_list::TodoList::new();
    load_from_local_storage(&mut todo_list);
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}TODO LIST\n\r{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        todo_list
    ).unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('j') => todo_list.select_previous_todo(),
            Key::Char('k') => todo_list.select_next_todo(),
            Key::Char('t') => todo_list.toggle_selected_todo(),
            _ => {}
        }
        stdout.flush().unwrap();

        write!(
            stdout,
            "{}{}TODO LIST\n\r{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            todo_list
        ).unwrap();

    }

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Show,
    ).unwrap();
}

fn load_from_local_storage(todo_list: &mut todo_list::TodoList) {
    let mut todo_file_path = dirs::config_dir().unwrap();
    todo_file_path.push(".todo_list");
    if let Ok(content) = fs::read_to_string(todo_file_path) {
        content.lines()
               .map(|line| {
                   let (completed, description) = line.split_at(1);
                   let completed = if completed == "1" { true } else { false };
                   todo::Todo::new(description.to_owned(), completed)
               }).for_each(|todo| {
                   todo_list.add(todo);
               });
    }
}
