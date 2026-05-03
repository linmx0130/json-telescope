use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use serde_json::Value;

use crate::json_tree::TreeState;

pub enum Screen {
    List,
    Inspect(TreeState),
}

pub struct App {
    pub entries: Vec<Value>,
    pub screen: Screen,
    pub list_selected: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new(entries: Vec<Value>) -> Self {
        Self {
            entries,
            screen: Screen::List,
            list_selected: 0,
            should_quit: false,
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        let mut back_to_list = false;
        match &mut self.screen {
            Screen::List => self.handle_list_key(key),
            Screen::Inspect(tree) => back_to_list = handle_inspect_key(tree, key),
        }
        if back_to_list {
            self.screen = Screen::List;
        }
    }

    fn handle_list_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.should_quit = true,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.list_selected > 0 {
                    self.list_selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.list_selected + 1 < self.entries.len() {
                    self.list_selected += 1;
                }
            }
            KeyCode::Enter | KeyCode::Char('l') => {
                if let Some(entry) = self.entries.get(self.list_selected).cloned() {
                    self.screen = Screen::Inspect(TreeState::new(entry));
                }
            }
            _ => {}
        }
    }
}

fn handle_inspect_key(tree: &mut TreeState, key: KeyEvent) -> bool {
    let visible = tree.flatten();
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return true,
        KeyCode::Up | KeyCode::Char('k') => tree.up(),
        KeyCode::Down | KeyCode::Char('j') => tree.down(visible.len()),
        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => tree.expand(&visible),
        KeyCode::Left | KeyCode::Char('h') => tree.collapse(&visible),
        _ => {}
    }
    false
}
