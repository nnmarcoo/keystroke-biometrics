use eframe::egui::{ScrollArea, Ui};
use std::time::{Duration, Instant};

pub struct Data {
    pair_timings: Vec<(String, Duration)>, // Changed to Vec<(String, Duration)>
    last_char: Option<(char, Instant)>,
    corrections: i32,
}

impl Data {
    pub fn new() -> Self {
        Self {
            pair_timings: Vec::new(), // Initialize Vec
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

                if let Some((_, old_duration)) =
                    self.pair_timings.iter_mut().find(|(p, _)| *p == pair)
                {
                    *old_duration = (*old_duration + duration) / 2;
                } else {
                    self.pair_timings.push((pair.clone(), duration));
                }
            }
            self.last_char = Some((new_char, now));
        } else {
            self.last_char = None;
        }
    }

    pub fn add_error(&mut self) {
        self.corrections += 1;
    }

    pub fn reset_last_char(&mut self) {
        self.last_char = None;
    }

    pub fn pop(&mut self) {
        self.pair_timings.pop();
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
