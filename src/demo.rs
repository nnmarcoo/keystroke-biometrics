use std::collections::HashSet;

use crate::{data::Data, top_bar::render_top_bar, typing::render_typing, util::gen_passage};
use eframe::{
    egui::{CentralPanel, Context, Key, Separator, SidePanel, Widget},
    App, CreationContext, Frame,
};

pub struct Demo {
    pub passage: String,
    pub input: String,
    pub type_data: Data,

    pub previous_keys: HashSet<Key>,
    pub backspace_debounce: i32,
    pub username: String,
    pub word_count: usize,
    pub use_database: bool,
    pub is_distracted: bool,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            passage: gen_passage(25),
            input: String::new(),
            type_data: Data::new(),

            previous_keys: HashSet::new(),
            backspace_debounce: 0,
            username: String::new(),
            word_count: 25,
            use_database: false,
            is_distracted: true,
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
            render_top_bar(self, ui);
            Separator::default().ui(ui);
            render_typing(self, ui);
            Separator::default().ui(ui);

            SidePanel::left("user_data")
            .resizable(true)
            .width_range(ui.available_width() * 0.2..=ui.available_width() * 0.8)
            .default_width(ui.available_width() / 2.)
            .show_inside(ui, |ui| {
                self.type_data.render_data(ui);
                ui.allocate_space(ui.available_size());
            });
            
        });
    }
}
