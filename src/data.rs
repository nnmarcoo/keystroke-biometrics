use eframe::egui::{Grid, RichText, ScrollArea, Ui};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{constants::FONT_ID_12, demo::Demo};

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

    pub fn is_populated(&mut self) -> bool {
        return self.history.len() > 1;
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

    pub fn calculate_pairs(&mut self) -> HashMap<(char, char), Duration> {
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

pub fn render_data(app: &mut Demo, ui: &mut Ui) {
    let mut average_pairs = app
        .type_data
        .calculate_pairs()
        .into_iter()
        .collect::<Vec<_>>();

    match app.user_data_sort {
        0 => {
            average_pairs.sort_by(|a, b| a.0.cmp(&b.0));
        }
        1 => {
            average_pairs.sort_by(|a, b| b.0.cmp(&a.0));
        }
        2 => {
            average_pairs.sort_by(|a, b| a.1.cmp(&b.1));
        }
        3 => {
            average_pairs.sort_by(|a, b| b.1.cmp(&a.1));
        }
        _ => {}
    }

    ScrollArea::both().show(ui, |ui| {
        Grid::new("key_pairs_grid").striped(true).show(ui, |ui| {
            for ((key1, key2), duration) in &average_pairs {
                let duration_ms = duration.as_secs_f64() * 1000.0;
                let k1 = key1.to_ascii_uppercase();
                let k2 = key2.to_ascii_uppercase();

                let key_pair_display = format!("{} ➡ {} ", k1, k2);
                let duration_display = format!("{:.4}", duration_ms);
                let hover_text = format!("{}➡{} key pair", k1, k2);
                let duration_hover =
                    format!("{:.0}ms between the {}➡{} key pair", duration_ms, k1, k2);

                let pair_res = ui
                    .label(RichText::new(&key_pair_display).font(FONT_ID_12))
                    .on_hover_text(&hover_text);
                let time_res = ui
                    .label(RichText::new(&duration_display).font(FONT_ID_12))
                    .on_hover_text(&duration_hover);
                ui.end_row();

                if pair_res.clicked() {
                    if app.user_data_sort != 0 {
                        app.user_data_sort = 0;
                    } else {
                        app.user_data_sort = 1;
                    }
                } else if time_res.clicked() {
                    if app.user_data_sort != 2 {
                        app.user_data_sort = 2;
                    } else {
                        app.user_data_sort = 3;
                    }
                }
            }
        });
    });
}
