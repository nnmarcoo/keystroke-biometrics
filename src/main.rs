#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate diesel;

mod constants;
mod data;
mod demo;
mod toggle_switch;
mod top_bar;
mod typing;
mod util;
mod schema;
mod db;
mod models;
mod ops;

use demo::Demo;
use eframe::{
    egui::{Style, ViewportBuilder, Visuals},
    Error, NativeOptions, Result,
};
use util::load_icon;

fn main() -> Result<(), Error> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800., 600.])
            .with_min_inner_size([400., 300.])
            .with_title("Keystroke Biometrics Demo")
            .with_icon(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "test",
        native_options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            Ok(Box::new(Demo::new(cc)))
        }),
    )
}
