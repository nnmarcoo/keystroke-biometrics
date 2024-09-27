use crate::util::gen_passage;
use eframe::{
    egui::{pos2, Align2, CentralPanel, Color32, Context, FontId, TextEdit, Ui},
    App, CreationContext, Frame,
};

pub struct Demo {
    passage: String,
    input: String,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            passage: gen_passage(),
            input: String::new(),
        }
    }
}

impl Demo {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self::default()
    }

    fn typing_ui(&mut self, ui: &mut Ui) {
        let painter = ui.painter();
        let mut x = 0.;
        let mut y = 50.;

        let font_id = FontId::monospace(16.);
        let char_spacing = 10.0;
        let available_width = ui.available_width();

        let mut input_chars = self.input.chars().peekable();

        let mut input_index = 0;

        // Define softer colors for green and red
        let soft_green = Color32::from_rgb(119, 221, 119); // #77dd77
        let soft_red = Color32::from_rgb(255, 105, 97);    // #ff6961

        for word in self.passage.split_whitespace() {
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

            if input_index < self.input.len() {
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
        ui.add_sized(
            [ui.available_width(), 16.],
            TextEdit::singleline(&mut self.input).hint_text("Start typing here"),
        );
    }
}

impl App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.typing_ui(ui);
        });
    }
}
