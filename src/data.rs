use eframe::egui::{ScrollArea, Ui};
use std::time::{Duration, Instant};

pub struct Data {
    history: Vec<(char, Instant)>,
    breaks: i32,
    corrections: i32,
}

impl Data {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            breaks: 0,
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
                self.breaks += 1;
            }
        }
    }

    pub fn reset(&mut self) {
        self.history.clear();
        self.breaks = 0;
        self.corrections = 0;
    }

    pub fn render_data(&mut self, ui: &mut Ui) {
        let chars: f32 = self.history.len() as f32 - self.breaks as f32;
        let foc: f32 = if chars != 0. {
            self.corrections as f32 / chars
        } else {
            0.
        };

        ui.label(format!("FOC: {}", foc));

        let res = ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
            for i in self.history.iter() {
                ui.label(format!("{}:{:?}", i.0, i.1));
            }
        });
    }
}
