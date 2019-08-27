extern crate dirs;

use std::fs;
use std::io;
use crate::todo::Todo;
use crate::app_state::AppState;

pub fn load_from_file(app_state: &mut AppState) -> io::Result<()> {
    let mut todo_file_path = dirs::config_dir().unwrap();
    todo_file_path.push(".todo_list");
    let content = fs::read_to_string(todo_file_path)?;
    content.lines()
           .map(|line| Todo::build_from_storage(line))
           .for_each(|todo| app_state.add(todo));
    Ok(())
}

pub fn save_to_file(app_state: &AppState) -> io::Result<()> {
    let mut todo_file_path = dirs::config_dir().unwrap();
    todo_file_path.push(".todo_list");
    let content = app_state.to_storage_content();
    fs::write(todo_file_path, content)?;
    Ok(())
}
