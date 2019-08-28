extern crate termion;
extern crate tui;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
//use termion::screen::AlternateScreen;
use termion::input::MouseTerminal;
use std::io::{/*Write,*/ stdout, stdin, Error};
use tui::Terminal;
use tui::layout::{Constraint, Direction, Layout};
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, SelectableList};
use tui::style::*;

mod todo;
mod handler;
mod app_state;

fn main() -> Result<(), Error> {
    let mut app_state = app_state::AppState::new();
    app_state.load();
    let mut handler = handler::Handler::new(&mut app_state);
    let stdin = stdin();
    let stdout = stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
//    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    for k in stdin.keys() {
        if let Ok(Key::Char('q')) = k { break };
        handler.handle_event(k.unwrap());
        let items = app_state.to_string_list();
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(80),
                    Constraint::Percentage(20)
                ].as_ref())
                .split(f.size());
            SelectableList::default()
                .block(Block::default().title("TO-DO LIST").borders(Borders::ALL))
                .items(&items)
                .select(Some(1))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().modifier(Modifier::ITALIC))
                .highlight_symbol(">>")
                .render(&mut f, chunks[0]);
            Block::default()
                .title("info")
                .borders(Borders::ALL)
                .render(&mut f, chunks[1]);
        })?;
    }
    app_state.persist();
    Ok(())
}
