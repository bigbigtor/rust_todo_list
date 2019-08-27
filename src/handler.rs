use termion::event::Key;
use crate::app_state::AppState;

pub struct Handler {
    app_state: &AppState,
}

impl Handler {
    pub fn new(appst: &AppState) -> Handler {
        Handler {
            app_state: appst,
        }
    }

    pub fn handle_event(&self, key: Key) {
        match key {
            Key::Char('q') => break,
            Key::Char('j') => self.app_state.select_previous_todo(),
            Key::Char('k') => self.app_state.select_next_todo(),
            Key::Char('t') => self.app_state.toggle_selected_todo(),
            _ => {}
        }
    }
}
