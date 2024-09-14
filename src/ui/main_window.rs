use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MainWindow {
    left_list: ListState,
    left_items: Vec<String>,
    right_list: ListState,
    right_items: Vec<String>,
}

impl MainWindow {
    pub fn new() -> Self {
        MainWindow {
            left_list: ListState::default(),
            left_items: vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string()],
            right_list: ListState::default(),
            right_items: Vec::new(),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        self.draw_left_list(f, chunks[0]);
        self.draw_right_list(f, chunks[1]);
    }

    fn draw_left_list<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let items: Vec<ListItem> = self
            .left_items
            .iter()
            .map(|i| ListItem::new(vec![Spans::from(Span::raw(i))]))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Left List"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        f.render_stateful_widget(list, area, &mut self.left_list);
    }

    fn draw_right_list<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let items: Vec<ListItem> = self
            .right_items
            .iter()
            .map(|i| ListItem::new(vec![Spans::from(Span::raw(i))]))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Right List"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        f.render_stateful_widget(list, area, &mut self.right_list);
    }

    pub fn on_left_select(&mut self) {
        if let Some(selected) = self.left_list.selected() {
            self.right_items = vec![format!("Details for {}", self.left_items[selected])];
            self.right_list.select(Some(0));
        }
    }

    pub fn on_key(&mut self, key: char) {
        match key {
            'j' => {
                if let Some(selected) = self.left_list.selected() {
                    if selected < self.left_items.len() - 1 {
                        self.left_list.select(Some(selected + 1));
                        self.on_left_select();
                    }
                } else {
                    self.left_list.select(Some(0));
                    self.on_left_select();
                }
            }
            'k' => {
                if let Some(selected) = self.left_list.selected() {
                    if selected > 0 {
                        self.left_list.select(Some(selected - 1));
                        self.on_left_select();
                    }
                } else {
                    self.left_list.select(Some(0));
                    self.on_left_select();
                }
            }
            _ => {}
        }
    }
}
