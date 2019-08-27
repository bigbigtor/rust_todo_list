use std::fmt;

#[derive(Debug)]
pub struct Todo {
    description: String,
    completed: bool,
}

impl Todo {
    pub fn new(desc: String, comp: bool) -> Todo {
        Todo {
            description: desc,
            completed: comp,
        }
    }

    pub fn toggle_complete(&self) -> Todo {
        Todo {
            description: self.description.to_owned(),
            completed: !self.completed,
        }
    }

    pub fn build_from_storage(line: &str) -> Todo {
       let (completed, description) = line.split_at(1);
       let completed = if completed == "1" { true } else { false };
       Todo::new(description.to_owned(), completed)
    }

    pub fn to_storage_line(&self) -> String {
        let comp = if self.completed { "1" } else { "0" };
        comp.to_owned() + &self.description
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut  fmt::Formatter) -> fmt::Result {
        let checkbox_fill = if self.completed { "x" } else { " " };
        write!(f, "[{}] {}", checkbox_fill, self.description)
    }
}
