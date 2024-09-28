use crate::{data::Data, typing::render_typing, util::gen_passage};
use eframe::{
    egui::{CentralPanel, Context},
    App, CreationContext, Frame,
};

pub struct Demo {
    pub passage: String,
    pub input: String,
    pub previous_length: usize,
    pub removed_char: char,
    pub type_data: Data,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            passage: gen_passage(),
            input: String::new(),
            previous_length: 0,
            removed_char: char::REPLACEMENT_CHARACTER,
            type_data: Data::new(),
        }
    }
}

impl Demo {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let y = render_typing(self, ui);
            ui.add_space(y - 16.);
            self.type_data.render_data(ui);
        });
    }
}
