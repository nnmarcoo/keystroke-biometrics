use crate::{
    demo::Demo,
    toggle_switch::toggle,
    util::{gen_passage, key_to_char},
};
use eframe::egui::{
    pos2, vec2, Align2, Button, Color32, DragValue, FontId, Key, Separator, Stroke, TextEdit, Ui,
    Widget,
};

pub fn render_typing(app: &mut Demo, ui: &mut Ui) {
    let mut is_username_focused = false;
    let toggle_text = if app.use_database {
        "Upload data to local database"
    } else {
        "Store data in-app (lost on exit)"
    };

    ui.horizontal(|ui| {
        is_username_focused = ui
            .add_sized(
                [100., 16.],
                TextEdit::singleline(&mut app.username).hint_text("Enter name"),
            )
            .on_hover_text("Who is typing")
            .has_focus();

        if ui
            .add_enabled(
                app.username.len() > 0,
                Button::new("⏵").min_size(vec2(16., 16.)),
            )
            .on_hover_text("Submit data")
            .clicked()
        {
            if app.username.len() > 0 {
                todo!("type_data.send");
            }
        }

        ui.add(toggle(&mut app.use_database))
            .on_hover_text(toggle_text);

        ui.add_space(ui.available_width() - 94.);

        if ui
            .add(DragValue::new(&mut app.word_count).range(1..=100))
            .on_hover_text("Passage length")
            .changed()
        {
            app.passage = gen_passage(app.word_count);
            app.input.clear();
        }

        if ui
            .add_enabled(true, Button::new("⟲").min_size(vec2(16., 16.)))
            .on_hover_text("Generate new passage")
            .clicked()
        {
            app.passage = gen_passage(app.word_count);
            app.input.clear();
        }

        if ui
            .add_enabled(true, Button::new("🗙").min_size(vec2(16., 16.)))
            .on_hover_text("Reset data")
            .clicked()
        {
            app.input.clear();
            app.previous_length = 0;
            app.type_data.reset();
        }
    });

    Separator::default().ui(ui);

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
        if is_username_focused {
            return;
        }

        let current_keys = i.keys_down.clone();
        let new_keys = current_keys.difference(&app.previous_keys);

        if current_keys.contains(&Key::Backspace) {
            app.backspace_debounce += 1;
            if app.backspace_debounce == 4 {
                app.backspace_debounce = 0;
                if app.input.len() > 1 {
                    let last_two = {
                        let split = app.input.char_indices().nth_back(1).unwrap().0;
                        &app.input[split..]
                    };
                    if !last_two.contains(' ') {
                        app.type_data.pop();
                    }
                }
                app.type_data.reset_last_char();
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
                if app.input.len() > 1 {
                    let last_two = {
                        let split = app.input.char_indices().nth_back(1).unwrap().0;
                        &app.input[split..]
                    };
                    if !last_two.contains(' ') {
                        app.type_data.pop();
                    }
                }
                app.type_data.reset_last_char();
                app.input.pop();
            }
        }
        app.previous_keys = current_keys;
    });

    ui.add_space(y - 30.);
    Separator::default().ui(ui);
}
