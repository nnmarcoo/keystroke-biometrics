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
    ];
    // lol change this
    let passage: Vec<&str> = words
        .choose_multiple(&mut rand::thread_rng(), 1)
        .cloned()
        .collect();
    passage.join(" ")
}
