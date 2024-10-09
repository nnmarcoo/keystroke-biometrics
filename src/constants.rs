use eframe::egui::{Color32, FontId};
pub const DATABASE_URL: &str = "mysql://root@localhost/keys";

pub const SOFT_GREEN: Color32 = Color32::from_rgb(119, 221, 119); // #77dd77
pub const SOFT_RED: Color32 = Color32::from_rgb(255, 105, 97); // #ff6961
pub const SOFT_YELLOW: Color32 = Color32::from_rgb(255, 230, 102); // #ffe666

pub const FONT_ID_16: FontId = FontId::monospace(16.);
pub const FONT_ID_12: FontId = FontId::monospace(12.);
pub const CHAR_SPACING: f32 = 10.;

pub const COLORS: [Color32; 11] = [
        Color32::from_rgb(255, 100, 100), // Red
        Color32::from_rgb(100, 255, 100), // Green
        Color32::from_rgb(100, 100, 255), // Blue
        Color32::from_rgb(255, 255, 100), // Yellow
        Color32::from_rgb(255, 100, 255), // Magenta
        Color32::from_rgb(100, 255, 255), // Cyan
        Color32::from_rgb(255, 165, 0),   // Orange
        Color32::from_rgb(75, 0, 130),    // Indigo
        Color32::from_rgb(255, 20, 147),  // Deep Pink
        Color32::from_rgb(173, 255, 47),  // Green Yellow
        Color32::from_rgb(0, 255, 255),   // Aqua
    ];
