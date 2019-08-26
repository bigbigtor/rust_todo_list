use std::fmt;

#[derive(Debug)]
pub struct Todo {
    description: String,
    completed: bool,
}

impl Todo {
    pub fn new(desc: String) -> Todo {
        Todo {
            description: desc,
            completed: false,
        }
    }

    pub fn toggle_complete(&self) -> Todo {
        Todo {
            description: self.description.to_owned(),
            completed: !self.completed,
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut  fmt::Formatter) -> fmt::Result {
        let checkbox_fill = if self.completed { "x" } else { " " };
        write!(f, "[{}] {}", checkbox_fill, self.description)
    }
}
