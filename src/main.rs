#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod data;
mod demo;
mod typing;
mod util;

use demo::Demo;
use eframe::{
    egui::{IconData, ViewportBuilder},
    run_native, Error, NativeOptions, Result,
};

fn main() -> Result<(), Error> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800., 600.])
            .with_min_inner_size([400., 300.])
            .with_title("Keystroke Biometrics Demo")
            .with_icon(IconData::default()),
        ..Default::default()
    };
    run_native(
        "Keystroke Biometrics Demo",
        native_options,
        Box::new(|cc| Ok(Box::new(Demo::new(cc)))),
    )
}
