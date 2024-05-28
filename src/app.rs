use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use noaa_spc_rss_parser::get_warnings;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::tui;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let KeyCode::Char('q') = key_event.code {
            self.exit()
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" NOAA SPC Watcher ".bold());
        let instructions = Title::from(Line::from(vec![
            " Scroll down ".into(),
            "<Down>".blue().bold(),
            " Scroll up ".into(),
            "<Up>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let warnings = get_warnings().unwrap();
        let mut text = Vec::new();

        for w in warnings {
            text.push(Line::from(vec!["Warning: ".into(), w.title.yellow()]));
        }

        Paragraph::new(Text::from(text))
            .centered()
            .block(block)
            .render(area, buf);
    }
}
