use crate::{demo::Demo, util::gen_passage};
use eframe::egui::{pos2, Align2, Button, Color32, FontId, TextEdit, Ui};

pub fn render_typing(app: &mut Demo, ui: &mut Ui) -> f32 {
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

    ui.horizontal(|ui| {
        if ui
            .add_sized(
                [ui.available_width() - 28., 16.],
                TextEdit::singleline(&mut app.input).hint_text("Start typing here"),
            )
            .changed()
        {
            let last_char = app.input.chars().last();
            if let Some(last_char) = last_char {
                let input_length = app.input.len();
                if input_length > app.previous_length {
                    app.type_data.record_char(last_char);
                } else {
                    app.type_data.add_error();
                    app.type_data
                        .remove_pair(&format!("{}{}", last_char, app.removed_char));
                }
                app.removed_char = last_char;
                app.previous_length = input_length;
            } else {
                app.type_data.reset();
            }
        }

        if ui
            .add_sized([16., 16.], Button::new("⟲"))
            .on_hover_text("Generate new passage")
            .clicked()
        {
            app.passage = gen_passage();
            app.input.clear();
            app.type_data.reset();
            app.previous_length = 0;
            app.removed_char = char::REPLACEMENT_CHARACTER;
        }
    });
    y
}
