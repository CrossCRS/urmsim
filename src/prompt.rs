use std::{collections::VecDeque, io};
use dialoguer::{theme::ColorfulTheme, History, Input};
use owo_colors::OwoColorize;

pub struct Prompt {
    history: CmdHistory,
}

impl Prompt {
    pub fn new() -> Self {
        println!("{} {} (c) {}", "URMSim".bright_blue().bold(), env!("CARGO_PKG_VERSION"), "Norbert Budzynski 2023".bright_white().bold());
        println!();

        return Self { history: CmdHistory::default() };
    }

    pub fn read(&mut self, pc: usize) -> io::Result<String> {
        return Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("URMSim (PC: {})", pc))
            .history_with(&mut self.history)
            .interact_text();
    }
}

struct CmdHistory {
    max: usize,
    history: VecDeque<String>,
}

impl Default for CmdHistory {
    fn default() -> Self {
        CmdHistory {
            max: 4,
            history: VecDeque::new(),
        }
    }
}

impl<T: ToString> History<T> for CmdHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        if self.history.len() == self.max {
            self.history.pop_back();
        }
        self.history.push_front(val.to_string());
    }
}