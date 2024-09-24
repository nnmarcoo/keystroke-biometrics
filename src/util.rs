use rand::seq::SliceRandom;

pub fn gen_passage(length: i32) -> String {
    let words = vec!["hi", "this", "is", "a", "test!"];
    let passage: Vec<_> = words
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();

    "".to_string()
}