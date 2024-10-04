use std::collections::HashSet;

use crate::{
    data::{render_data, Data},
    top_bar::render_top_bar,
    typing::render_typing,
    util::gen_passage,
};
use eframe::{
    egui::{CentralPanel, Context, Key, Separator, SystemTheme, ViewportCommand, Widget},
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
    pub user_data_sort_mode: bool,
    pub fullscreen: bool,
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
            user_data_sort_mode: true,
            fullscreen: false,
        }
    }
}

impl Demo {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        cc.egui_ctx.send_viewport_cmd(ViewportCommand::SetTheme(SystemTheme::Dark));
        cc.egui_ctx.send_viewport_cmd(ViewportCommand::Visible(false));
        cc.egui_ctx.send_viewport_cmd(ViewportCommand::Visible(true));
        Self::default()
    }
}

impl App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {

        if ctx.input(|i| i.key_pressed(Key::F11)) {
            self.fullscreen = !self.fullscreen;
            ctx.send_viewport_cmd(ViewportCommand::Fullscreen(self.fullscreen));
        }

        CentralPanel::default().show(ctx, |ui| {
            render_top_bar(self, ui);
            Separator::default().ui(ui);
            render_typing(self, ui);
            Separator::default().ui(ui);

            ui.horizontal_top(|ui| {
                render_data(self, ui);
                if self.type_data.is_populated() {
                    Separator::default().vertical().ui(ui);
                }
            });
        });
    }
}
