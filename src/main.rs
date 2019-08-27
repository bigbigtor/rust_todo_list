extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

mod todo;
mod todo_list;
mod storage;

fn main() {
    let mut todo_list = todo_list::TodoList::new();
    storage::load_from_file(&mut todo_list);
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}TODO LIST\n\r{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
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
            "{}{}{}TODO LIST\n\r{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
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
