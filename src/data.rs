use eframe::egui::{Grid, ScrollArea, Ui};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

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
        let mut average_pairs = self.calculate_pairs().into_iter().collect::<Vec<_>>();
        average_pairs.sort_by(|a, b| a.0.cmp(&b.0));

        ScrollArea::both().show(ui, |ui| {
            Grid::new("key_pairs_grid").striped(true).show(ui, |ui| {
                ui.label("Pair");
                ui.label("Duration (ms)");
                ui.end_row();

                for ((key1, key2), duration) in average_pairs.iter() {
                    let duration_ms = duration.as_secs_f64() * 1000.0;
                    ui.label(format!("{} ➡ {}", key1, key2));
                    ui.label(format!("{:.4}", duration_ms));
                    ui.end_row();
                }
            });
        });
    }

    fn calculate_pairs(&mut self) -> HashMap<(char, char), Duration> {
        let mut pair_durations: HashMap<(char, char), (Duration, usize)> = HashMap::new();

        for window in self.history.windows(2) {
            let (key1, time1) = window[0];
            let (key2, time2) = window[1];

            if key1 == '_' || key1 == ' ' || key2 == '_' || key2 == ' ' {
                continue;
            }

            let duration = time2.duration_since(time1);

            let entry = pair_durations
                .entry((key1, key2))
                .or_insert((Duration::new(0, 0), 0));
            entry.0 += duration;
            entry.1 += 1;
        }

        let mut average_durations: HashMap<(char, char), Duration> = HashMap::new();

        for (pair, (total_duration, count)) in pair_durations {
            let average_duration = total_duration / count as u32;
            average_durations.insert(pair, average_duration);
        }
        average_durations
    }
}
