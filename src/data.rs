use eframe::egui::{Grid, RichText, ScrollArea, Separator, Ui, Widget};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{constants::FONT_ID_12, demo::Demo};

pub struct Data {
    history: Vec<(char, Instant)>,
    breaks: i32,
    corrections: i32,
    pairs: Arc<Mutex<HashMap<(char, char), Duration>>>,
    cpe: Arc<Mutex<f32>>,
    wpm: Arc<Mutex<f32>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            breaks: 0,
            corrections: 0,
            pairs: Arc::new(Mutex::new(HashMap::new())),
            cpe: Arc::new(Mutex::new(0.)),
            wpm: Arc::new(Mutex::new(0.)),
        }
    }

    pub fn record_char(&mut self, key: char) {
        self.history.push((key, Instant::now()));
        self.update_pairs();
    }

    pub fn pop(&mut self) {
        self.history.pop();
        self.corrections += 1;
        self.update_pairs();
    }

    pub fn is_populated(&mut self) -> bool {
        return self
            .history
            .iter()
            .filter(|&&c| c.0.is_alphabetic())
            .count()
            > 1;
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
        self.update_pairs();
    }

    pub fn update_pairs(&self) {
        let pairs_clone = Arc::clone(&self.pairs);
        let history_clone = self.history.clone();

        let wpm_clone = Arc::clone(&self.wpm);
        let cpe_clone = Arc::clone(&self.cpe);
        let corrections = self.corrections;

        thread::spawn(move || {
            let mut pair_durations = HashMap::new();

            for window in history_clone.windows(2) {
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

            let mut average_durations = HashMap::new();

            for (pair, (total_duration, count)) in pair_durations {
                let average_duration = total_duration / count as u32;
                average_durations.insert(pair, average_duration);
            }

            let mut pairs_lock = pairs_clone.lock().unwrap_or_else(|e| e.into_inner());
            *pairs_lock = average_durations;

            if let (Some(first), Some(last)) = (history_clone.first(), history_clone.last()) {
                let first_time = first.1;
                let last_time = last.1;
                let total_time = last_time.duration_since(first_time).as_secs_f32() / 60.0; // time in minutes
                let char_count = history_clone
                    .iter()
                    .filter(|&&(c, _)| c.is_alphabetic())
                    .count();
                let wpm = (char_count as f32 / 5.0) / total_time;

                let mut cpe = if char_count > 0 {
                    corrections as f32 / char_count as f32
                } else {
                    0.0
                };
                cpe = cpe * 100.;

                let mut wpm_lock = wpm_clone.lock().unwrap_or_else(|e| e.into_inner());
                *wpm_lock = wpm;

                let mut foc_lock = cpe_clone.lock().unwrap_or_else(|e| e.into_inner());
                *foc_lock = cpe;
            }
        });
    }

    pub fn get_pairs(&self) -> Arc<Mutex<HashMap<(char, char), Duration>>> {
        Arc::clone(&self.pairs)
    }

    pub fn get_cpe(&self) -> Arc<Mutex<f32>> {
        Arc::clone(&self.cpe)
    }

    pub fn get_wpm(&self) -> Arc<Mutex<f32>> {
        Arc::clone(&self.wpm)
    }
}

pub fn render_data(app: &mut Demo, ui: &mut Ui) {
    if !app.type_data.is_populated() {
        return;
    }

    let pairs_lock = app.type_data.get_pairs();
    let average_pairs = pairs_lock.lock().unwrap().clone();
    let mut sorted_pairs = average_pairs.into_iter().collect::<Vec<_>>();

    if app.user_data_sort_mode {
        sorted_pairs.sort_by(|a, b| a.0.cmp(&b.0));
    } else {
        sorted_pairs.sort_by(|a, b| a.1.cmp(&b.1));
    }

    let wpm_lock = app.type_data.get_wpm();
    let cpe_lock = app.type_data.get_cpe();
    let wpm = wpm_lock.lock().unwrap().clone();
    let cpe = cpe_lock.lock().unwrap().clone();

    ScrollArea::vertical().show(ui, |ui| {
        Grid::new("key_pairs_grid").striped(true).show(ui, |ui| {
            ui.label(RichText::new("WPM").font(FONT_ID_12))
                .on_hover_text("Words per minute");
            ui.label(RichText::new(format!("{:.4}", wpm)).font(FONT_ID_12))
                .on_hover_text(format!("{:.0} words per minute", wpm));
            ui.end_row();

            ui.label(RichText::new("CPE").font(FONT_ID_12))
                .on_hover_text("% likelihood of mistake per character");
            ui.label(RichText::new(format!("{:.4}", cpe)).font(FONT_ID_12))
                .on_hover_text(format!(
                    "{:.0}% chance that you make a mistake per character",
                    cpe
                ));
            ui.end_row();

            Separator::default().ui(ui);
            Separator::default().ui(ui);
            ui.end_row();

            for ((key1, key2), duration) in &sorted_pairs {
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

                if pair_res.clicked() || time_res.clicked() {
                    app.user_data_sort_mode = !app.user_data_sort_mode;
                }
            }
        });
    });
}
