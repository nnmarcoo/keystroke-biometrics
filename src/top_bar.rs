use crate::{ops::{create_user, get_users, insert_metrics, insert_pairs}, util::{get_match, get_selected_points}};
use eframe::egui::{vec2, Button, DragValue, TextEdit, Ui};

use crate::{db::establish_connection, demo::Demo, toggle_switch::toggle, util::gen_passage};

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
                if app.use_database {
                    let id = create_user(&app.username.to_lowercase()).unwrap();
                    app.username.clear();
                    insert_pairs(id, &app.type_data);
                    insert_metrics(
                        id,
                        app.type_data.get_wpm_value(),
                        app.type_data.get_cpe_value(),
                    );
                    app.users = get_users().unwrap();
                }
            }
        }

        if ui
            .add(toggle(&mut app.use_database))
            .on_hover_text(toggle_text)
            .changed()
        {
            if app.use_database {
                if let Some(_) = establish_connection() {
                    app.users = get_users().unwrap();
                    app.match_and_counts = get_match(&app.type_data).unwrap();
                } else {
                    app.use_database = false;
                }
            }
        }

        if ui
            .add_enabled(
                app.use_database,
                Button::new("🔄").min_size(vec2(16., 16.)),
            )
            .on_hover_text("Refresh users")
            .clicked()
        {
            app.users = get_users().unwrap();
        }

        ui.add_space(ui.available_width() - 95.);

        if ui
            .add(DragValue::new(&mut app.word_count).range(1..=200))
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
            .on_hover_text("Clear client data")
            .clicked()
        {
            app.input.clear();
            app.type_data.reset();
        }
    });
}
