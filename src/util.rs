use eframe::egui::{pos2, Color32, IconData, Key, Painter, Pos2, Stroke};
use image::load_from_memory;
use rand::seq::SliceRandom;

use crate::constants;

pub fn gen_passage(length: usize) -> String {
    let words = vec![
        "apple", "bottle", "create", "dragon", "energy", "family", "giant", "hollow", "iceberg", "jungle",
        "kitten", "lemon", "mountain", "notion", "orange", "python", "quiver", "raven", "system", "tiger",
        "umbrella", "vivid", "whisper", "xenon", "yellow", "zebra", "anchor", "butter", "clown", "dynamo",
        "eagle", "frost", "grape", "hammer", "island", "jacket", "kettle", "lantern", "monster", "novel",
        "ocean", "puzzle", "quartz", "rabbit", "shadow", "tornado", "unicorn", "valley", "wizard", "xerox",
        "yawn", "zephyr", "abyss", "balance", "circle", "daring", "echo", "feather", "glimmer", "horizon",
        "ignite", "journey", "keypad", "library", "motion", "network", "oasis", "panic", "radiant", "signal",
        "tempo", "utopia", "venture", "whale", "yodel", "zealot", "arch", "biscuit", "cloud", "disaster",
        "ember", "feast", "glory", "harvest", "impact", "joker", "knot", "light", "moment", "nature", "option",
        "pillar", "query", "rescue", "shield", "track", "union", "vector", "whimsy", "zone", "arrow", "bubble",
        "candle", "dust", "envy", "flash", "gleam", "habit", "ink", "juggle", "kiosk", "link", "mirror", "navy",
        "orchid", "plume", "quilt", "razor", "siren", "trick", "under", "victor", "xray", "yearn", "zero",
        "angle", "branch", "cabin", "delta", "flame", "gloom", "hatch", "ivory", "jigsaw", "kite", "leaf",
        "mask", "naval", "pistol", "quote", "river", "spark", "tide", "velvet", "wind", "yarn", "zinc", "aura",
        "blend", "crane", "delight", "essence", "flock", "gaze", "haste", "idea", "joint", "magnet", "neutral",
        "outlook", "pulse", "quasar", "rush", "sprint", "truth", "uphold", "vortex", "warp", "zeal", "beacon",
        "castle", "dash", "eclipse", "fume", "glow", "herd", "inkling", "jewel", "kingdom", "loft", "mystic",
        "night", "plank", "quest", "ridge", "shine", "tangle", "uplift", "vista", "whistle", "yield", "zenith",
        "bold", "crisp", "dusty", "freight", "glider", "hurdle", "kick", "lively", "mild", "noisy", "oath",
        "pounce", "quiet", "riot", "sleek", "tough", "vault", "wrest", "yolk", "axis", "breeze", "crash",
        "dive", "flare", "hush", "lure", "mist", "nudge", "quench", "reel", "silk", "uplift", "yodel"
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
            pos2(pos.x + 10., pos.y + constants::CHAR_SPACING),
        ],
        Stroke::new(2., color),
    );
}