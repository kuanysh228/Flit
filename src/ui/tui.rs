use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use ratatui::crossterm::event::{self, Event, KeyEventKind};
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::crossterm::ExecutableCommand;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

use crate::error::FlitError;
use crate::ui::app::App;
use crate::ui::input::action::Action;
use crate::ui::input::command::parse_command;
use crate::ui::input::keys::handle_key;
use crate::ui::input::mode::Mode;
use crate::ui::render::context::ContextWidget;
use crate::ui::render::guide::GuideWidget;
use crate::ui::render::help::HelpWidget;
use crate::ui::render::status::StatusWidget;
use crate::ui::render::word::WordWidget;

const TICK: Duration = Duration::from_millis(16);

pub fn run(mut app: App) -> Result<Option<u64>> {
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = event_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn event_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<Option<u64>> {
    let mut last_word: Option<crate::core::word::Word> = None;

    loop {
        let now = Instant::now();

        if let Some(word) = app.engine.tick(now) {
            last_word = Some(word.clone());
        }

        let cols = terminal.size()?.width;
        if cols < 40 {
            return Err(FlitError::TerminalTooNarrow.into());
        }
        let pivot_col = cols / 3;

        terminal.draw(|frame| {
            let area = frame.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(area);

            frame.render_widget(
                GuideWidget { pivot_col, theme: &app.theme },
                chunks[0],
            );

            if let Some(ref w) = last_word {
                frame.render_widget(
                    WordWidget { word: w, pivot_col, theme: &app.theme },
                    chunks[1],
                );
            }

            if app.engine.is_paused() {
                let ctx = app.engine.context_words(2);
                if ctx.len() >= 1 {
                    frame.render_widget(
                        ContextWidget { words: &ctx[ctx.len().saturating_sub(1)..], theme: &app.theme },
                        chunks[3],
                    );
                }
                if ctx.len() >= 2 {
                    frame.render_widget(
                        ContextWidget { words: &ctx[..1], theme: &app.theme },
                        chunks[4],
                    );
                }
            }

            frame.render_widget(
                StatusWidget {
                    wpm: app.engine.wpm(),
                    progress: app.engine.progress_percent(),
                    elapsed_secs: app.elapsed_secs(),
                    paused: app.engine.is_paused(),
                    theme: &app.theme,
                },
                chunks[6],
            );

            if app.show_help {
                let help_area = centered_rect(60, 80, area);
                frame.render_widget(HelpWidget, help_area);
            }
        })?;

        if event::poll(TICK)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if app.show_help {
                    app.show_help = false;
                    continue;
                }

                let action = handle_key(&mut app.mode, key);

                if let Some(action) = action {
                    match &action {
                        Action::EnterCommandMode => {
                            app.mode = Mode::Command { buffer: String::new() };
                        }
                        Action::EnterSearchMode => {
                            app.mode = Mode::Search { buffer: String::new(), forward: true };
                        }
                        Action::ShowHelp => { app.show_help = true; }
                        Action::ShowStats => { app.show_stats = !app.show_stats; }
                        Action::Quit => {
                            return Ok(Some(app.engine.current_position().index));
                        }
                        Action::QuitNoSave => { return Ok(None); }
                        _ => { app.engine.apply(action, now); }
                    }
                }

                if let Mode::Command { buffer } = &app.mode {
                    if let Some(act) = parse_command(buffer) {
                        let buf = buffer.clone();
                        app.mode = Mode::default();
                        match act {
                            Action::Quit => return Ok(Some(app.engine.current_position().index)),
                            Action::QuitNoSave => return Ok(None),
                            Action::ShowStats => { app.show_stats = true; }
                            other => { app.engine.apply(other, now); }
                        }
                        let _ = buf;
                    }
                }
            }
        }

        if app.engine.is_finished() {
            return Ok(Some(app.engine.current_position().index));
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
