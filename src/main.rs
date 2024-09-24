#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod demo;

use demo::Demo;
use eframe::{egui::{IconData, ViewportBuilder}, run_native, Error, NativeOptions, Result};

fn main() -> Result<(), Error> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("Keystroke Biometrics Demo"),
        ..Default::default()
    };
    run_native(
        "Keystroke Biometrics Demo",
        native_options,
        Box::new(|cc| Ok(Box::new(Demo::new(cc)))),
    )
}