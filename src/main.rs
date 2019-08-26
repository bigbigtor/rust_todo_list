extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

mod todo;
mod todo_list;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let todo = todo::Todo::new(String::from("hola"));
    let todo2 = todo::Todo::new(String::from("macaco"));
    let todo3 = todo::Todo::new(String::from("do selva"));
    let todo = todo.complete();
    let mut todo_list = todo_list::TodoList::new();
    todo_list.add(todo);
    todo_list.add(todo2);
    todo_list.add(todo3);
    println!("{}", todo_list);

    //write!(
    //    stdout,
    //    "{}{}TODO LIST{}",
    //    termion::clear::All,
    //    termion::cursor::Goto(1, 1),
    //    termion::cursor::Hide
    //).unwrap();
    //stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        ).unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Esc => println!("ESC"),
            Key::Char('j') => {
                if todo_list.get_selected_index() < todo_list.get_len() {
                    todo_list.set_selected_index(todo_list.get_selected_index() + 1);
                }
            },
            Key::Char('k') => {
                if 0 < todo_list.get_selected_index() {
                    todo_list.set_selected_index(todo_list.get_selected_index() - 1);
                }
            },
            _ => {}
        }
        stdout.flush().unwrap();
        write!(stdout, "{}", todo_list);

    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
