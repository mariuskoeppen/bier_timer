use uuid::Uuid;

use super::{Fluid, Temperature};

#[derive(Debug, Clone)]
/// Can be a fridge, freezer, or others
pub struct Ambience {
    pub id: Uuid,
    pub name: String,
    pub path_to_image: String,
    pub temperature: Temperature,
    pub fluid: Fluid,
}

impl Ambience {
    pub fn new(
        name: &str,
        path_to_image: &str,
        temperature: Temperature,
        fluid: Option<Fluid>,
    ) -> Self {
        Ambience {
            id: Uuid::new_v4(),
            name: String::from(name),
            path_to_image: String::from(path_to_image),
            temperature,
            fluid: fluid.unwrap_or(Fluid::Air),
        }
    }
}
