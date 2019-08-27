extern crate dirs;
use std::fs;
use crate::todo::Todo;
use crate::todo_list::TodoList;

pub fn load_from_file(todo_list: &mut TodoList) {
    let mut todo_file_path = dirs::config_dir().unwrap();
    todo_file_path.push(".todo_list");
    if let Ok(content) = fs::read_to_string(todo_file_path) {
        content.lines()
               .map(|line| Todo::build_from_storage(line))
               .for_each(|todo| todo_list.add(todo));
    }
}

pub fn save_to_file(todo_list: &TodoList) {
    let mut todo_file_path = dirs::config_dir().unwrap();
    todo_file_path.push(".todo_list");
    let content = todo_list.to_storage_content();
    fs::write(todo_file_path, content);
}
