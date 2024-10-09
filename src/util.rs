use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use eframe::egui::{
    pos2, Color32, Grid, IconData, Key, Painter, Pos2, RichText, ScrollArea, Stroke, Ui,
};
use egui_plot::{Bar, BarChart, Legend, Orientation, Plot, Points};
use image::load_from_memory;
use rand::seq::SliceRandom;

use crate::{
    constants::{self, COLORS, FONT_ID_12, SOFT_GREEN},
    data::{build_points_from_durations, Data},
    demo::Demo,
    ops::{get_metrics, get_pairs, match_metrics, match_pairs, remove_user},
};

pub fn gen_passage(length: usize) -> String {
    let words = vec![
        "apple", "bottle", "create", "dragon", "energy", "family", "giant", "hollow", "iceberg",
        "jungle", "kitten", "lemon", "mountain", "notion", "orange", "python", "quiver", "raven",
        "system", "tiger", "umbrella", "vivid", "whisper", "xenon", "yellow", "zebra", "anchor",
        "butter", "clown", "dynamo", "eagle", "frost", "grape", "hammer", "island", "jacket",
        "kettle", "lantern", "monster", "novel", "ocean", "puzzle", "quartz", "rabbit", "shadow",
        "tornado", "unicorn", "valley", "wizard", "xerox", "yawn", "zephyr", "abyss", "balance",
        "circle", "daring", "echo", "feather", "glimmer", "horizon", "ignite", "journey", "keypad",
        "library", "motion", "network", "oasis", "panic", "radiant", "signal", "tempo", "utopia",
        "venture", "whale", "yodel", "zealot", "arch", "biscuit", "cloud", "disaster", "ember",
        "feast", "glory", "harvest", "impact", "joker", "knot", "light", "moment", "nature",
        "option", "pillar", "query", "rescue", "shield", "track", "union", "vector", "whimsy",
        "zone", "arrow", "bubble", "candle", "dust", "envy", "flash", "gleam", "habit", "ink",
        "juggle", "kiosk", "link", "mirror", "navy", "orchid", "plume", "quilt", "razor", "siren",
        "trick", "under", "victor", "xray", "yearn", "zero", "angle", "branch", "cabin", "delta",
        "flame", "gloom", "hatch", "ivory", "jigsaw", "kite", "leaf", "mask", "naval", "pistol",
        "quote", "river", "spark", "tide", "velvet", "wind", "yarn", "zinc", "aura", "blend",
        "crane", "delight", "essence", "flock", "gaze", "haste", "idea", "joint", "magnet",
        "neutral", "outlook", "pulse", "quasar", "rush", "sprint", "truth", "uphold", "vortex",
        "warp", "zeal", "beacon", "castle", "dash", "eclipse", "fume", "glow", "herd", "inkling",
        "jewel", "kingdom", "loft", "mystic", "night", "plank", "quest", "ridge", "shine",
        "tangle", "uplift", "vista", "whistle", "yield", "zenith", "bold", "crisp", "dusty",
        "freight", "glider", "hurdle", "kick", "lively", "mild", "noisy", "oath", "pounce",
        "quiet", "riot", "sleek", "tough", "vault", "wrest", "yolk", "axis", "breeze", "crash",
        "dive", "flare", "hush", "lure", "mist", "nudge", "quench", "reel", "silk", "uplift",
        "yodel",
    ];

    let mut rng = rand::thread_rng();
    let passage: Vec<&str> = words.choose_multiple(&mut rng, length).cloned().collect();

    passage.join(" ")
}

pub fn key_to_char(key: Key) -> Option<char> {
    match key {
        Key::A => Some('a'),
        Key::B => Some('b'),
        Key::C => Some('c'),
        Key::D => Some('d'),
        Key::E => Some('e'),
        Key::F => Some('f'),
        Key::G => Some('g'),
        Key::H => Some('h'),
        Key::I => Some('i'),
        Key::J => Some('j'),
        Key::K => Some('k'),
        Key::L => Some('l'),
        Key::M => Some('m'),
        Key::N => Some('n'),
        Key::O => Some('o'),
        Key::P => Some('p'),
        Key::Q => Some('q'),
        Key::R => Some('r'),
        Key::S => Some('s'),
        Key::T => Some('t'),
        Key::U => Some('u'),
        Key::V => Some('v'),
        Key::W => Some('w'),
        Key::X => Some('x'),
        Key::Y => Some('y'),
        Key::Z => Some('z'),
        Key::Space => Some(' '),
        _ => None,
    }
}

pub fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/icon_128.png");
        let image = load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

pub fn draw_cursor(painter: &Painter, pos: Pos2, color: Color32) {
    painter.line_segment(
        [
            pos2(pos.x, pos.y + constants::CHAR_SPACING),
            pos2(
                pos.x + constants::CHAR_SPACING,
                pos.y + constants::CHAR_SPACING,
            ),
        ],
        Stroke::new(2., color),
    );
}

pub fn render_users(app: &mut Demo, ui: &mut Ui) {
    ScrollArea::vertical()
        .id_salt("users_scroll")
        .show(ui, |ui| {
            Grid::new("users_grid").striped(true).show(ui, |ui| {
                let create_context_menu = |ui: &mut Ui, name: String, id: i32| {
                    if ui.button(format!("Delete {}", name)).clicked() {
                        let _ = remove_user(id);
                        ui.close_menu();
                    }
                };

                for u in app.users.iter() {
                    if app.match_and_counts.2.contains_key(&u.0) {
                        let mut color = Color32::GRAY;
                        if u.0 == app.match_and_counts.0 {
                            color = SOFT_GREEN;
                        }

                        let v = *app.match_and_counts.2.get(&u.0).unwrap() as f32;
                        let p = v / (app.match_and_counts.1 as f32) * 100.;
                        let user = &(u.0, u.1.clone());

                        let name_res = ui
                            .selectable_label(
                                app.selected_users.contains(user),
                                RichText::new(&u.1).font(FONT_ID_12).color(color),
                            )
                            .on_hover_text(&format!("ID: {}", u.0));

                        ui.label(
                            RichText::new(format!("{:.2}%", p))
                                .font(FONT_ID_12)
                                .color(color),
                        )
                        .on_hover_text(&format!("{} / {}", v, app.match_and_counts.1));

                        ui.end_row();

                        name_res.context_menu(|ui| create_context_menu(ui, u.1.clone(), u.0));

                        if name_res.clicked() {
                            if app.selected_users.contains(user) {
                                app.selected_users.remove(user);
                            } else {
                                app.selected_users.insert(user.clone());
                            }
                            app.selected_points = get_selected_points(app.type_data.get_pairs_copy(), app.selected_users.clone());
                        }
                    }
                }

                for u in app.users.iter() {
                    if !app.match_and_counts.2.contains_key(&u.0) {
                        let user = &(u.0, u.1.clone());

                        let name_res = ui
                            .selectable_label(
                                app.selected_users.contains(user),
                                RichText::new(&u.1).font(FONT_ID_12).color(Color32::GRAY),
                            )
                            .on_hover_text(&format!("ID: {}", u.0));

                        ui.label(RichText::new("0.00%").font(FONT_ID_12).color(Color32::GRAY))
                            .on_hover_text("0 / 0");
                        ui.end_row();

                        name_res.context_menu(|ui| create_context_menu(ui, u.1.clone(), u.0));

                        if name_res.clicked() {
                            if app.selected_users.contains(user) {
                                app.selected_users.remove(user);
                            } else {
                                app.selected_users.insert(user.clone());
                            }
                            app.selected_points = get_selected_points(app.type_data.get_pairs_copy(), app.selected_users.clone());
                        }
                    }
                }
            });
        });
}

pub fn render_charts(app: &Demo, ui: &mut Ui) {
    let mut bars: Vec<Bar> = Vec::new();
    let mut x = 0.;

    bars.push(new_bar(
        String::from("Entry WPM"),
        x,
        app.type_data.get_wpm_value() as f64,
        Color32::GRAY,
        0.,
    ));
    bars.push(new_bar(
        String::from("entry CPE"),
        x,
        app.type_data.get_cpe_value() as f64,
        Color32::GRAY,
        app.type_data.get_wpm_value() as f64 + 1.,
    ));
    x += 1.5;

    for (i, u) in app.selected_users.iter().enumerate() {
        if let Ok(Some(metrics)) = get_metrics(u.0) {
            let color = COLORS[i % COLORS.len()];

            bars.push(new_bar(
                format!("{} WPM", u.1.clone()),
                x,
                metrics.0 as f64,
                color,
                0.,
            ));
            bars.push(new_bar(
                format!("{} CPE", u.1.clone()),
                x,
                metrics.1 as f64,
                color,
                metrics.0 as f64 + 1.,
            ));
            x += 1.5;
        }
    }

    ui.vertical(|ui| {
        Plot::new("Metrics Plot")
            .height(ui.available_height() / 2.)
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(BarChart::new(bars).name("Metrics"));
            });

        Plot::new("Pairs Plot").legend(Legend::default()).show(ui, |plot_ui| {
            for (i, p) in app.selected_points.iter().enumerate() {
                let mut color = Color32::GRAY;
                if p.0 != "Entry" {
                    color = COLORS[i % COLORS.len()];
                }

                let points = Points::new(p.1.clone())
                    .color(color)
                    .radius(3.)
                    .name(p.0.clone());
                plot_ui.points(points);
            }
        });
    });
}

pub fn get_match(type_data: &Data) -> Option<(i32, i32, HashMap<i32, usize>)> {
    let mut pairs = match_pairs(type_data);
    let metrics = match_metrics(type_data).unwrap();

    pairs.entry(metrics.0).and_modify(|e| *e += 5).or_insert(1);
    pairs.entry(metrics.1).and_modify(|e| *e += 5).or_insert(1);

    let max_key = pairs.iter().max_by_key(|&(_, v)| v).map(|(&k, _)| k)?;
    let total_count: i32 = pairs.values().sum::<usize>() as i32;

    Some((max_key, total_count, pairs))
}

fn new_bar(text: String, pos: f64, height: f64, color: Color32, offset: f64) -> Bar {
    Bar {
        orientation: Orientation::Vertical,
        name: text,
        argument: pos,
        value: height,
        base_offset: Some(offset),
        bar_width: 1.,
        stroke: Stroke::NONE,
        fill: color,
    }
}

pub fn get_selected_points(
    entry_data: Vec<(String, Duration)>,
    users: HashSet<(i32, String)>,
) -> Vec<(String, Vec<[f64; 2]>)> {
    let mut points = Vec::new();

    let entry_without_duration: Vec<String> = entry_data.iter().map(|(s, _)| s.clone()).collect();

    for u in users.iter() {
        let mut x = 0.;
        let query_pairs = get_pairs(u.0, entry_without_duration.clone()).unwrap();
        let mut pairs_vec: Vec<[f64; 2]> = Vec::new();

        for p in query_pairs.iter() {
            pairs_vec.push([x, *p as f64]);
            x += 1.;
        }
        points.push((u.1.clone(), pairs_vec));
    }

    points.push((String::from("Entry"), build_points_from_durations(&entry_data)));
    points
}
