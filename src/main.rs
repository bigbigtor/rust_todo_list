extern crate termion;
extern crate tui;

//use termion::event::Key;
//use termion::input::TermRead;
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
mod storage;

fn main() -> Result<(), Error> {
    let mut app_state = app_state::AppState::new();
    let handler = handler::Handler::new(&app_state);
    storage::load_from_file(&mut app_state)?;
    let stdin = stdin();
    let stdout = stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
//    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    for k in stdin.keys() {

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
                .items(
                    &todo_list.iter()
                              .map(|t| format!("{}", t))
                              .collect::<Vec<String>>()
                )
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
    storage::save_to_file(&app_state)?;
    Ok(())
}
