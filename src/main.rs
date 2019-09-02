extern crate termion;
extern crate tui;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::input::MouseTerminal;
use std::io::{stdout, stdin, Error};
use tui::{Terminal, Frame};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, SelectableList, Paragraph, Text};
use tui::style::*;

mod todo;
mod app_state;

fn main() -> Result<(), Error> {
    let mut app_state = app_state::AppState::new();
    app_state.load()?;
    let stdin = stdin();
    let stdout = stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    terminal.autoresize()?;
    draw(&mut terminal, &app_state)?;
    for k in stdin.keys() {
        match app_state.mode {
            app_state::Mode::Normal => {
                if let Ok(Key::Char('q')) = k { break };
                terminal.hide_cursor()?;
            },
            app_state::Mode::Insert => {
                terminal.show_cursor()?;
            },
        }
        app_state.handle_event(k.unwrap());
        draw(&mut terminal, &app_state)?;
    }
    app_state.persist()?;
    Ok(())
}

fn draw<B>(terminal: &mut Terminal<B>, app_state: &app_state::AppState) -> Result<(), Error>
where
    B: Backend,
{
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(98),
                Constraint::Min(2),
            ].as_ref())
            .split(f.size());
        draw_list(&mut f, chunks[0], &app_state);
        draw_info_block(&mut f, chunks[1], &app_state);
    })?;
    Ok(())
}

fn draw_list<B>(f: &mut Frame<B>, layout: Rect, app_state: &app_state::AppState)
where
    B: Backend,
{
    let items = app_state.todos
                         .iter()
                         .map(|t| format!("{}", t))
                         .collect::<Vec<String>>();

    SelectableList::default()
        .block(
            Block::default()
            .title("TO-DO LIST")
            .borders(Borders::ALL)
        )
        .items(&items)
        .select(Some(app_state.selected_index))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
            .fg(Color::Black)
            .bg(Color::White)
        )
        .render(f, layout);
}

fn draw_info_block<B>(f: &mut Frame<B>, layout: Rect, app_state: &app_state::AppState)
where
    B: Backend,
{
    let lines = [
        Text::styled(
            format!(" {:?} ", app_state.mode).to_uppercase(),
            Style::default().fg(Color::Black).bg(Color::White).modifier(Modifier::BOLD)
        ),
        Text::raw(
            " j: down, k: up, t: toggle, i: insert, a: append"
        ),
    ];
    Paragraph::new(lines.iter()).render(f, layout);
}
