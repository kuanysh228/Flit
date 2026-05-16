use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::action::Action;
use super::mode::Mode;

pub fn handle_key(mode: &mut Mode, key: KeyEvent) -> Option<Action> {
    match mode {
        Mode::Normal { count } => handle_normal(count, key),
        Mode::Command { buffer } => handle_command(buffer, key),
        Mode::Search { buffer, forward } => handle_search(buffer, *forward, key),
        Mode::Pause => handle_pause(key),
    }
}

fn handle_normal(count: &mut Option<u64>, key: KeyEvent) -> Option<Action> {
    if key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT {
        match key.code {
            KeyCode::Char(' ') => return Some(Action::TogglePause),
            KeyCode::Char('q') => return Some(Action::Quit),
            KeyCode::Char(':') => return Some(Action::EnterCommandMode),
            KeyCode::Char('/') => return Some(Action::EnterSearchMode),
            KeyCode::Char('?') => return Some(Action::ShowHelp),

            KeyCode::Char('j') | KeyCode::Char('[') => {
                return Some(Action::ChangeSpeed(0.9))
            }
            KeyCode::Char('k') | KeyCode::Char(']') => {
                return Some(Action::ChangeSpeed(1.1))
            }

            KeyCode::Char('h') => {
                let n = count.take().unwrap_or(1) as i64;
                return Some(Action::SeekWords(-n));
            }
            KeyCode::Char('l') => {
                let n = count.take().unwrap_or(1) as i64;
                return Some(Action::SeekWords(n));
            }
            KeyCode::Char('b') => {
                let n = count.take().unwrap_or(1) as i64;
                return Some(Action::SeekSentence(-n));
            }
            KeyCode::Char('w') => {
                let n = count.take().unwrap_or(1) as i64;
                return Some(Action::SeekSentence(n));
            }
            KeyCode::Char('{') => {
                let n = count.take().unwrap_or(1) as i64;
                return Some(Action::SeekParagraph(-n));
            }
            KeyCode::Char('}') => {
                let n = count.take().unwrap_or(1) as i64;
                return Some(Action::SeekParagraph(n));
            }

            KeyCode::Char('G') => return Some(Action::GotoEnd),
            KeyCode::Char('n') => return Some(Action::SearchNext),
            KeyCode::Char('N') => return Some(Action::SearchPrev),

            KeyCode::Char(c @ '0'..='9') => {
                let digit = c.to_digit(10).unwrap() as u64;
                *count = Some(count.unwrap_or(0) * 10 + digit);
                return None;
            }
            KeyCode::Char('g') => {
                if *count == Some(0) {
                    *count = None;
                    return Some(Action::GotoStart);
                }
            }

            KeyCode::Char('m') => {
                *count = None;
            }

            _ => {
                *count = None;
            }
        }
    }
    None
}

fn handle_command(buffer: &mut String, key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Enter => Some(Action::EnterCommandMode),
        KeyCode::Esc => {
            buffer.clear();
            None
        }
        KeyCode::Backspace => {
            buffer.pop();
            None
        }
        KeyCode::Char(c) => {
            buffer.push(c);
            None
        }
        _ => None,
    }
}

fn handle_search(buffer: &mut String, _forward: bool, key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Enter => Some(Action::SearchNext),
        KeyCode::Esc => {
            buffer.clear();
            None
        }
        KeyCode::Backspace => {
            buffer.pop();
            None
        }
        KeyCode::Char(c) => {
            buffer.push(c);
            None
        }
        _ => None,
    }
}

fn handle_pause(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char(' ') => Some(Action::TogglePause),
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('?') => Some(Action::ShowHelp),
        _ => None,
    }
}
