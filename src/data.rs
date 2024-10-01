use eframe::egui::{ScrollArea, Ui};
use std::time::{Duration, Instant};

pub struct Data {
    history: Vec<(char, Instant)>,
    corrections: i32,
}

impl Data {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            corrections: 0,
        }
    }

    pub fn record_char(&mut self, key: char) {
        self.history.push((key, Instant::now()));
    }

    pub fn pop(&mut self) {
        self.history.pop();
        self.corrections += 1;
    }

    pub fn insert_break(&mut self) {
        if self.history.len() > 0 {
            if self.history.last().unwrap().0 != '_' {
                self.history.push(('_', Instant::now()));
            }
        }
    }

    pub fn reset(&mut self) {
        self.history.clear();
        self.corrections = 0;
    }

    pub fn render_data(&mut self, ui: &mut Ui) {
        ui.label(format!("Corrections: {}", self.corrections));
        ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
            for i in self.history.iter() {
                ui.label(format!("{}:{:?}", i.0, i.1));
            }
        });
    }
}
