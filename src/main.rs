#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod demo;
mod util;

use demo::Demo;
use eframe::{
    egui::{IconData, ViewportBuilder},
    run_native, Error, NativeOptions, Result,
};

fn main() -> Result<(), Error> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
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
