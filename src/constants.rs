use eframe::egui::{Color32, FontId};
pub const DATABASE_URL: &str = "mysql://root@localhost/keys";

pub const SOFT_GREEN: Color32 = Color32::from_rgb(119, 221, 119); // #77dd77
pub const SOFT_RED: Color32 = Color32::from_rgb(255, 105, 97); // #ff6961
pub const SOFT_YELLOW: Color32 = Color32::from_rgb(255, 230, 102); // #ffe666

pub const FONT_ID_16: FontId = FontId::monospace(16.);
pub const FONT_ID_12: FontId = FontId::monospace(12.);
pub const CHAR_SPACING: f32 = 10.;
