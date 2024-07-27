mod app;
mod components;
pub mod helpers;
mod pages;

pub use self::{app::App, calculate_surface_area::*, helpers::*, pages::*};

pub mod calculate_surface_area {
    use std::f64::consts::PI;
    use uom::si::f64::{Area, Length};
    // use uom::si::length::{centimeter, meter, millimeter};
    pub fn area_of_cylindric_container(diameter: Length, height: Length) -> Area {
        // 2πr(h+r)
        let radius = diameter / 2.0;
        2.0 * PI * radius * (height + radius) - PI * radius * radius
    }

    pub fn area_of_bottle_container(
        diameter_top_bottle: Length,
        height_tapered_neck: Length,
        diameter_base: Length,
        height_base: Length,
    ) -> Area {
        let a = diameter_top_bottle / 2.0;
        let b = diameter_base / 2.0;
        let h1 = height_base;
        let h2 = height_tapered_neck;

        PI * (a + b) * (h2 * h2 + (b - a) * (b - a)).sqrt() + 2.0 * PI * b * h1 + PI * b * b
    }
}
