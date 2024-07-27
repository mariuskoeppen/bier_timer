use super::{ambience::Ambience, drink::Drink};

#[derive(Clone)]
// I always assume intial temp to be room temp and ambience to be freezer, so i could get rid of them
pub struct TimerPreset {
    pub name: String,
    pub path_to_image: String,
    pub drink: Drink,
    pub initial_ambience: Ambience,
    pub ambient_ambience: Ambience,
    pub target_ambience: Ambience,
}
