use std::{sync::mpsc, thread};

use crate::{
    constants,
    demo::Demo,
    util::{draw_cursor, gen_passage, get_match, get_selected_points, key_to_char},
};
use eframe::egui::{pos2, Align2, Color32, Key, Ui};

pub fn render_typing(app: &mut Demo, ui: &mut Ui) {
    if app.input == app.passage {
        app.passage = gen_passage(app.word_count);
        app.input.clear();

        if app.input.len() == 0 {
            app.type_data.insert_break();
        }
    }

    let painter = ui.painter();
    let mut pos = pos2(0., 50.);

    let available_width = ui.available_width();

    let input_chars: Vec<char> = app.input.chars().collect();
    let passage_chars: Vec<char> = app.passage.chars().collect();
    let mut i = 0;

    for word in app.passage.split_whitespace() {
        let word_width = word.chars().count() as f32 * constants::CHAR_SPACING;

        if pos.x + word_width > available_width {
            pos.x = 0.;
            pos.y += 25.;
        }

        for c in word.chars() {
            pos.x += constants::CHAR_SPACING;

            let color = if i < input_chars.len() {
                if input_chars[i] == c {
                    constants::SOFT_GREEN
                } else {
                    constants::SOFT_RED
                }
            } else {
                Color32::GRAY
            };

            if i == app.input.len() {
                draw_cursor(painter, pos, constants::SOFT_YELLOW);
            }

            painter.text(
                pos,
                Align2::LEFT_CENTER,
                c,
                constants::FONT_ID_16.clone(),
                color,
            );
            i += 1;
        }
        pos.x += constants::CHAR_SPACING;

        if i == app.input.len() {
            draw_cursor(painter, pos, constants::SOFT_YELLOW);
        } else if i < input_chars.len() {
            if input_chars[i] != passage_chars[i] {
                draw_cursor(painter, pos, constants::SOFT_RED);
            }
        }

        i += 1;
    }

    ui.add_space(4.);

    ui.input(|i| {
        if app.is_distracted {
            return;
        }

        let current_keys = i.keys_down.clone();
        let new_keys = current_keys.difference(&app.previous_keys);

        if current_keys.contains(&Key::Backspace) && current_keys.len() == 1 {
            app.backspace_debounce += 1;
            if app.backspace_debounce > 4 {
                app.backspace_debounce = 0;
                if app.input.len() > 0 {
                    app.type_data.pop();
                }
                app.input.pop();
            }
        }

        for key in new_keys {
            if *key != Key::Backspace {
                if let Some(ch) = key_to_char(*key) {
                    if app.input.len() < app.passage.len() {
                        app.type_data.record_char(ch);
                        app.input.push(ch);
                    }
                }
            } else if *key == Key::Backspace {
                app.backspace_debounce = 0;
                if app.input.len() > 0 {
                    app.type_data.pop();
                }
                app.input.pop();
            }

            if app.use_database {
                let (tx, rx) = mpsc::channel();
                let type_data = app.type_data.clone();
                let selected_users = app.selected_users.clone();
            
                thread::spawn(move || {
                    let selected_points = get_selected_points(type_data.get_pairs_copy(), selected_users);
                    let result = get_match(&type_data).unwrap();
                    tx.send((selected_points, result)).unwrap();
                });
            
                if let Ok((selected_points, result)) = rx.recv() {
                    app.selected_points = selected_points;
                    app.match_and_counts = result;
                }
            }
        }
        app.previous_keys = current_keys;
    });

    ui.add_space(pos.y - 30.);
}
