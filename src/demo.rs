use std::collections::HashSet;

use crate::{data::Data, top_bar::render_top_bar, typing::render_typing, util::gen_passage};
use eframe::{
    egui::{vec2, CentralPanel, Context, Key, Separator, SidePanel, Widget},
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

            
           
        });
    }
}
