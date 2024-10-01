use crate::{demo::Demo, util::{gen_passage, key_to_char}};
use eframe::egui::{pos2, Align2, Color32, FontId, Key, Stroke, Ui};

pub fn render_typing(app: &mut Demo, ui: &mut Ui) {
    let painter = ui.painter();
    let mut x = 0.;
    let mut y = 50.;

    let font_id = FontId::monospace(16.);
    let char_spacing = 10.;
    let available_width = ui.available_width();

    let mut input_chars = app.input.chars().peekable();
    let mut input_index = 0;

    let soft_green = Color32::from_rgb(119, 221, 119); // #77dd77
    let soft_red = Color32::from_rgb(255, 105, 97); // #ff6961

    for word in app.passage.split_whitespace() {
        let word_width = word.chars().count() as f32 * char_spacing;

        if x + word_width > available_width {
            x = 0.;
            y += 25.;
        }

        for c in word.chars() {
            let typed_char = input_chars.peek();

            let color = if let Some(&typed) = typed_char {
                input_index += 1;
                if typed == c {
                    soft_green
                } else {
                    soft_red
                }
            } else {
                Color32::GRAY
            };

            painter.text(
                pos2(x + 10., y),
                Align2::LEFT_CENTER,
                c,
                font_id.clone(),
                color,
            );
            x += char_spacing;
            input_chars.next();
        }

        if input_index < app.input.len() {
            if let Some(&next_input_char) = input_chars.peek() {
                let color = if next_input_char == ' ' {
                    soft_green
                } else {
                    soft_red
                };
                painter.text(
                    pos2(x + 10., y),
                    Align2::LEFT_CENTER,
                    ' ',
                    font_id.clone(),
                    color,
                );
                input_chars.next();
                input_index += 1;
            }
        }
        x += char_spacing;
    }

    ui.add_space(4.);

    ui.input(|i| {
        if app.is_distracted {
            return;
        }

        let current_keys = i.keys_down.clone();
        let new_keys = current_keys.difference(&app.previous_keys);

        if current_keys.contains(&Key::Backspace) {
            app.backspace_debounce += 1;
            if app.backspace_debounce == 4 {
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
                    app.type_data.record_char(ch);
                    app.input.push(ch);
                }
            } else if *key == Key::Backspace {
                app.backspace_debounce = 0;
                if app.input.len() > 0 {
                    app.type_data.pop();
                }
                app.input.pop();
            }
        }
        app.previous_keys = current_keys;
    });

    if app.input == app.passage {
        app.passage = gen_passage(app.word_count);
        app.input.clear();

        if app.input.len() == 0 {
            app.type_data.insert_break();
        }
    }
    
    ui.add_space(y - 30.);
}
