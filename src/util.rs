use eframe::egui::Key;
use rand::seq::SliceRandom;

pub fn gen_passage() -> String {
    let words = vec![
        "amazingly bob can dance elegantly flipping gracefully near open ponds quick rabbits jumped keenly leaping many nearby obstacles portraying quiet resilience silly turtles undertake vast journeys exploring wild zones and beautiful colors",
        "alligators bask calmly diving effortlessly into flowing green habitats intrepid jaguars keep lookout moving nimbly observing playful quokkas resting silently turtles usually venture wandering xenophobic yet zealous",
        "bird can fly gracefully hovering in joyful kindness lending many new opportunities playful quokkas rest silently taking unique vacations while xerophytes yield zest",
        "curious dog explored fields gathering herbs and intriguing jellies keenly it leaped making new observations playing quietly under vibrant wildflowers xerophytes yielding zest",
        "fox galloped hastily ignoring jagged knolls lively mice navigated open paths quietly resting slowly transitioning under various willows xenophobic yet yielding zebras approached calmly",
        "always brave charlie dug eagerly for golden hidden jewels keeping low mice nibbled on peanuts quietly resting stealing treats under various wildflowers xenophobic yet zealous",
        "friendly gorilla hid in jungle knolls looking for mischievous nocturnal opossums playful quails roamed silently trailing under vines while xenophytes yielded zest",
        "gentle hen invited joyful kids lending many nutritious offerings peacocks quickly roamed showcasing their vibrant wings xerophytes yielding zest all around",
        "giant kangaroo leaped magnificently navigating open pastures quickly resting silently turtles utilized various warm zones yet quickly yielded zest",
        "jackal kept leaping mightily nimbly observing playful quokkas resting silently turtles undertook various wild journeys yielding zeal in their exploration",
        "this is a short test",
    ];
    // lol change this
    let passage: Vec<&str> = words
        .choose_multiple(&mut rand::thread_rng(), 1)
        .cloned()
        .collect();
    passage.join(" ")
}

pub fn key_to_char(key: Key) -> Option<char> {
    // This maps the key to a character; add more mappings as needed
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
        Key::Enter => Some('\n'),
        _ => None,
    }
}
