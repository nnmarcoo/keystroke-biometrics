use eframe::egui::{ScrollArea, Ui};
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Data {
    pair_timings: HashMap<String, Duration>,
    last_char: Option<(char, Instant)>,
    corrections: i32,
}

impl Data {
    pub fn new() -> Self {
        Self {
            pair_timings: HashMap::new(),
            last_char: None,
            corrections: 0,
        }
    }

    pub fn record_char(&mut self, new_char: char) {
        if new_char != ' ' {
            let now = Instant::now();
            if let Some((previous_char, previous_time)) = self.last_char {
                let duration = now.duration_since(previous_time);
                let pair = format!("{}{}", previous_char, new_char);
                self.pair_timings.insert(pair.clone(), duration);
            }
            self.last_char = Some((new_char, now));
        } else {
            self.last_char = None;
        }
    }

    pub fn add_error(&mut self) {
        self.corrections += 1;
    }

    pub fn remove_pair(&mut self, k: &String) {
        self.pair_timings.remove(k);
    }

    pub fn reset(&mut self) {
        self.pair_timings.clear();
        self.last_char = None;
        self.corrections = 0;
    }

    pub fn render_data(&mut self, ui: &mut Ui) {
        ui.label(format!("Corrections: {}", self.corrections));

        ScrollArea::vertical().show(ui, |ui| {
            for (pair, duration) in &self.pair_timings {
                ui.label(format!("{}: {:?}", pair, duration));
            }
        });
    }
}
