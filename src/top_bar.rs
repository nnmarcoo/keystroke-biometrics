use eframe::egui::{vec2, Button, DragValue, TextEdit, Ui};

use crate::{demo::Demo, toggle_switch::toggle, util::gen_passage};

pub fn render_top_bar(app: &mut Demo, ui: &mut Ui) {
    let toggle_text = if app.use_database {
        "Upload data to local database"
    } else {
        "Store data in-app (lost on exit)"
    };

    ui.horizontal(|ui| {
        app.is_distracted = ui
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
            app.type_data.insert_break();
            app.passage = gen_passage(app.word_count);
            app.input.clear();
        }

        if ui
            .add_enabled(true, Button::new("🗙").min_size(vec2(16., 16.)))
            .on_hover_text("Clear data")
            .clicked()
        {
            app.input.clear();
            app.type_data.reset();
        }
    });
}
