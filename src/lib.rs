mod app;
mod components;
mod pages;

pub use self::{app::App, pages::*};

pub mod utils {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

