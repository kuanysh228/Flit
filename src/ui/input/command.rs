use crate::ui::input::action::Action;

pub fn parse_command(input: &str) -> Option<Action> {
    let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();
    match parts.as_slice() {
        ["q"] | ["quit"] => Some(Action::Quit),
        ["q!"] => Some(Action::QuitNoSave),
        ["wq"] => Some(Action::Quit),
        ["set", "wpm", val] => val.parse::<u16>().ok().map(Action::SetWpm),
        ["goto", val] => {
            if let Some(pct) = val.strip_suffix('%') {
                pct.parse::<u8>().ok().map(Action::GotoPercent)
            } else {
                val.parse::<i64>().ok().map(|n| Action::SeekWords(n))
            }
        }
        ["stats"] => Some(Action::ShowStats),
        ["help"] => Some(Action::ShowHelp),
        _ => None,
    }
}
