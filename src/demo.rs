use eframe::{
    egui::{CentralPanel, Context},
    App, CreationContext, Frame,
};

pub struct Demo {
}

impl Default for Demo {
    fn default() -> Self {
        Self {}
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
            ui.label("Hello, world!");
        });
    }
}