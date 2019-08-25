//extern crate termion;
//
//use termion::event::Key;
//use termion::input::TermRead;
//use termion::raw::IntoRawMode;
//use std::io::{Write, stdout, stdin};

mod todo;
mod todo_list;

fn main() {
    //let stdin = stdin();
    //let mut stdout = stdout().into_raw_mode().unwrap();
    let todo = todo::Todo::new(String::from("hola"));
    let todo = todo.complete();
    let mut todo_list = todo_list::TodoList::new();
    todo_list.add(todo);
    println!("{}", todo_list);

    //write!(
    //    stdout,
    //    "{}{}TODO LIST{}",
    //    termion::clear::All,
    //    termion::cursor::Goto(1, 1),
    //    termion::cursor::Hide
    //).unwrap();
    //stdout.flush().unwrap();

    //for c in stdin.keys() {
    //    write!(
    //        stdout,
    //        "{}{}",
    //        termion::cursor::Goto(1, 1),
    //        termion::clear::CurrentLine
    //    ).unwrap();

    //    match c.unwrap() {
    //        Key::Char('q') => break,
    //        Key::Char(c) => println!("{}", c),
    //        Key::Alt(c) => println!("^{}", c),
    //        Key::Ctrl(c) => println!("*{}", c),
    //        Key::Esc => println!("ESC"),
    //        Key::Left => println!("<-"),
    //        Key::Right => println!("->"),
    //        Key::Up => println!("^"),
    //        Key::Down => println!("v"),
    //        Key::Backspace => println!("x"),
    //        _ => {}
    //    }
    //    stdout.flush().unwrap();


    //}

    //write!(stdout, "{}", termion::cursor::Show).unwrap();
}
