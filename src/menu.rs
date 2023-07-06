use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{self, Block, Borders, List, ListItem, ListState},
    Frame,
};

#[derive(Default)]
pub struct Menu {
    raw_items: Vec<Rc<String>>,
    state: RefCell<ListState>,
}

impl Menu {
    pub fn new(items: Vec<String>) -> Self {
        let raw_items = items.into_iter().map(Rc::new).collect();
        let state = RefCell::new(ListState::default());
        state.borrow_mut().select(Some(0));

        Self {
            raw_items,
            state,
            ..Default::default()
        }
    }

    fn current_index(&self) -> Option<usize> {
        self.state.borrow().selected()
    }

    pub fn select_next_item(&self) {
        let cur = self.current_index();

        if cur.is_none() {
            return;
        }

        let cur = cur.unwrap();

        let mut next = cur + 1;

        if next > self.items_count() - 1 {
            next = 0;
        }

        self.state.borrow_mut().select(Some(next))
    }

    pub fn select_prev_item(&self) {
        let cur = self.current_index();

        if cur.is_none() {
            return;
        }

        let cur = cur.unwrap();

        let prev = if cur == 0 {
            self.items_count() - 1
        } else {
            cur - 1
        };

        self.state.borrow_mut().select(Some(prev))
    }

    pub fn select_first_item(&self) {
        if self.current_index().is_none() {
            return;
        }

        self.state.borrow_mut().select(Some(0))
    }

    pub fn select_last_item(&self) {
        if self.current_index().is_none() {
            return;
        }

        self.state.borrow_mut().select(Some(self.items_count() - 1))
    }

    fn items_count(&self) -> usize {
        self.raw_items.len()
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let items = self
            .raw_items
            .iter()
            .map(|i| vec![Span::raw(i.as_str())])
            .collect::<Vec<_>>();

        let cur = self.current_index();

        if cur.is_none() {
            frame.render_widget(
                List::new([]).block(Block::default().borders(Borders::ALL)),
                area,
            );
            return;
        }

        let cur = cur.unwrap();

        let items = items
            .into_iter()
            .enumerate()
            .map(|(index, mut spans)| {
                if index == cur {
                    for span in spans.iter_mut() {
                        span.style.bg.get_or_insert(Color::LightBlue);
                        // FIXME: Upstream bug?
                        // Werid behavior in TTY when set color to white
                        span.style.fg.insert(Color::White);
                    }
                }

                ListItem::new(Line::from(spans))
            })
            .collect::<Vec<_>>();

        let list = widgets::List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_symbol("> ")
            .highlight_style(Style::default());

        frame.render_stateful_widget(list, area, &mut self.state.borrow_mut())
    }
}

pub struct MenuView {
    inner: Menu,
}

impl MenuView {
    pub fn new<I: Into<Vec<String>>>(items: I) -> Self {
        let inner = Menu::new(items.into());

        Self { inner }
    }

    pub fn on_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Up => {
                self.inner.select_prev_item();
            }
            KeyCode::Char('k') => {
                self.inner.select_prev_item();
            }
            KeyCode::Down => {
                self.inner.select_next_item();
            }
            KeyCode::Char('j') => {
                self.inner.select_next_item();
            }
            KeyCode::Home => {
                self.inner.select_first_item();
            }
            KeyCode::End => {
                self.inner.select_last_item();
            }
            _ => {}
        }
    }
}

impl Deref for MenuView {
    type Target = Menu;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for MenuView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub fn to_string_vec<'a, A: IntoIterator<Item = &'a str>>(arr: A) -> Vec<String> {
    arr.into_iter().map(|s| s.to_string()).collect()
}
