use termion::event::Key;
use crate::app_state::AppState;

pub struct Handler<'a> {
    app_state: &'a mut AppState,
}

impl<'a> Handler<'a> {
    pub fn new(appst: &'a mut AppState) -> Handler {
        Handler {
            app_state: appst,
        }
    }

    pub fn handle_event(&mut self, key: Key) {
        match key {
            Key::Char('j') => self.app_state.select_previous_todo(),
            Key::Char('k') => self.app_state.select_next_todo(),
            Key::Char('t') => self.app_state.toggle_selected_todo(),
            _ => {}
        }
    }
}
