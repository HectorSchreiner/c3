use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

use crate::log::*;

pub struct Interface;

impl Interface {
    pub fn init(logstorage: &mut LogStorage) -> io::Result<()> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        let app_result = Self::run(terminal, logstorage);
        ratatui::restore();
        app_result
    }

    fn run(mut terminal: DefaultTerminal, logstorage: &mut LogStorage) -> io::Result<()> {
        loop {
            terminal.draw(|frame| {
                let greeting = Paragraph::new("Cool C2 Server").white().on_dark_gray();
                frame.render_widget(greeting, frame.area());

                logstorage.add_log(C2Log::new(LogLevel::Info, "Created new Window".to_owned()));
            })?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
