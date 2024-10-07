use eframe::egui::{Grid, RichText, ScrollArea, Separator, Ui, Widget};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use std::fs::File;
use std::io::{self, Write};

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
        self.update_data();
    }

    pub fn pop(&mut self) {
        self.history.pop();
        self.corrections += 1;
        self.update_data();
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
        self.update_data();
    }

    pub fn update_data(&self) {
        let pairs_clone = Arc::clone(&self.pairs);
        let history_clone = self.history.clone();

        let wpm_clone = Arc::clone(&self.wpm);
        let cpe_clone = Arc::clone(&self.cpe);
        let corrections = self.corrections;

        thread::spawn(move || {
            let mut pair_durations = HashMap::new();
            let mut total_segment_time = Duration::new(0, 0);
            let mut segment_char_count = 0;

            let mut segment_start: Option<Instant> = None;

            for window in history_clone.windows(2) {
                let (key1, time1) = window[0];
                let (key2, time2) = window[1];

                if key1 == '_' {
                    segment_start = Some(time2);
                } else if key2 == '_' {
                    if let Some(start_time) = segment_start {
                        total_segment_time += time2.duration_since(start_time);
                        segment_start = None;
                    }
                } else if segment_start.is_none() {
                    segment_start = Some(time1);
                }

                if key1 != '_' && key2 != ' ' && key1 != ' ' && key2 != '_' {
                    let duration = time2.duration_since(time1);
                    let entry = pair_durations
                        .entry((key1, key2))
                        .or_insert((Duration::new(0, 0), 0));
                    entry.0 += duration;
                    entry.1 += 1;
                }

                if key1.is_alphabetic() {
                    segment_char_count += 1;
                }
            }

            if let Some(start_time) = segment_start {
                if let Some((_, last_time)) = history_clone.last() {
                    total_segment_time += last_time.duration_since(start_time);
                }
            }

            let mut average_durations = HashMap::new();
            for (pair, (total_duration, count)) in pair_durations {
                let average_duration = total_duration / count as u32;
                average_durations.insert(pair, average_duration);
            }

            let mut pairs_lock = pairs_clone.lock().unwrap_or_else(|e| e.into_inner());
            *pairs_lock = average_durations;

            if total_segment_time.as_secs_f32() > 0. && segment_char_count > 0 {
                let total_time_minutes = total_segment_time.as_secs_f32() / 60.;
                let wpm = (segment_char_count as f32 / 5.0) / total_time_minutes;

                let cpe = if segment_char_count > 0 {
                    corrections as f32 / segment_char_count as f32 * 100.
                } else {
                    0.
                };

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

    pub fn clean_pairs(&self, threshold: f64) -> HashMap<(char, char), Duration> {
        let pairs_lock = self.pairs.lock().unwrap();
        let pairs: HashMap<(char, char), Duration> = pairs_lock.clone();

        let durations: Vec<f64> = pairs
            .values()
            .map(|duration| duration.as_secs_f64() * 1000.0)
            .collect();

        if durations.is_empty() {
            return HashMap::new();
        }

        let mean = durations.iter().sum::<f64>() / durations.len() as f64;
        let variance = durations
            .iter()
            .map(|duration| (duration - mean).powi(2))
            .sum::<f64>()
            / durations.len() as f64;
        let std_dev = variance.sqrt();

        let lower_bound = mean - threshold * std_dev;
        let upper_bound = mean + threshold * std_dev;

        let cleaned_pairs = pairs
            .into_iter()
            .filter(|(_, duration)| {
                let duration_ms = duration.as_secs_f64() * 1000.;
                duration_ms >= lower_bound && duration_ms <= upper_bound
            })
            .collect::<HashMap<(char, char), Duration>>();

        cleaned_pairs
    }

    pub fn get_wpm_value(&self) -> f32 {
        let wpm_lock = self.wpm.lock().unwrap_or_else(|e| e.into_inner());
        *wpm_lock
    }

    pub fn get_cpe_value(&self) -> f32 {
        let cpe_lock = self.cpe.lock().unwrap_or_else(|e| e.into_inner());
        *cpe_lock
    }

    pub fn export_to_csv(&self) -> Result<(), io::Error> {
        let mut file = File::create("export.csv")?;

        writeln!(file, "WPM,CPE")?;

        let wpm_value = self.get_wpm_value();
        let cpe_value = self.get_cpe_value();
        writeln!(file, "{:.4},{:.4}", wpm_value, cpe_value)?;

        let pairs_lock = self.get_pairs();
        let pairs = pairs_lock.lock().unwrap();

        writeln!(file, "Pair,Interval (ms)")?;

        for ((key1, key2), duration) in pairs.iter() {
            let pair = format!("{}{}", key1, key2);
            let duration_ms = duration.as_secs_f64() * 1000.0;

            writeln!(file, "{}, {:.4}", pair, duration_ms)?;
        }
        Ok(())
    }

    pub fn export_to_sql(&self) -> Result<String, io::Error> {
        let mut sql_statements = String::new();

        let wpm_value = self.get_wpm_value();
        let cpe_value = self.get_cpe_value();

        sql_statements.push_str("INSERT INTO USER (name) VALUES ('<NAME>');\n\n");

        sql_statements.push_str(&format!(
            "INSERT INTO METRICS (id, wpm, cpe) VALUES (?, {:.4}, {:.4});\n\n",
            wpm_value, cpe_value
        ));

        let pairs_lock = self.get_pairs();
        let pairs = pairs_lock.lock().unwrap();

        for ((key1, key2), duration) in pairs.iter() {
            let pair = format!("{}{}", key1, key2);
            let duration_ms = duration.as_secs_f64() * 1000.0;

            sql_statements.push_str(&format!(
                "INSERT INTO PAIRS (id, pair, `interval`) VALUES (?, '{}', {:.4});\n",
                pair, duration_ms
            ));
        }

        let mut file = File::create("export.sql")?;
        file.write_all(sql_statements.as_bytes())?;

        Ok(sql_statements)
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

    ScrollArea::vertical()
        .id_source("user_data_scroll")
        .show(ui, |ui| {
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

                    let key_pair_display = format!("{} ➡ {}", k1, k2);
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

                    let create_context_menu = |ui: &mut Ui| {
                        if ui.button("Export to .sql").clicked() {
                            let _ = app.type_data.export_to_sql();
                            ui.close_menu();
                        }
                        if ui.button("Export to .csv").clicked() {
                            let _ = app.type_data.export_to_csv();
                            ui.close_menu();
                        }
                    };

                    pair_res.context_menu(|ui| create_context_menu(ui));
                    time_res.context_menu(|ui| create_context_menu(ui));
                }
            });
        });
}
