use crate::util::gen_passage;
use eframe::{
    egui::{pos2, Align2, CentralPanel, Color32, Context, FontId, Label, RichText, Sense, Ui}, emath::Float, App, CreationContext, Frame
};

pub struct Demo {
    passage: String,
    input: String,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            passage: gen_passage(15),
            input: String::new(),
        }
    }
}

impl Demo {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self::default()
    }

    fn typing_ui(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            for (i, target_char) in self.passage.chars().enumerate() {
                let painter = ui.painter();

            let pos = pos2((i as f32) * 16., 25.);

            painter.text(pos, Align2::LEFT_CENTER, target_char, FontId::monospace(16.), Color32::GRAY);
            }
        });

        ui.text_edit_singleline(&mut self.input);
    }
    
}

impl App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.typing_ui(ui);
        });
    }
}
