#![allow(unused)]
use bier_timer::*;
use leptos::*;
use uom::si::length::{centimeter, millimeter};

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> });
    use uom::si::f64::Length;
    leptos::logging::log!(
        "{:#?}",
        area_of_cylindric_container(
            Length::new::<millimeter>(66.0),
            Length::new::<millimeter>(168.0)
        )
    );

    leptos::logging::log!(
        "{:#?}",
        area_of_bottle_container(
            Length::new::<centimeter>(3.0),
            Length::new::<centimeter>(7.0),
            Length::new::<millimeter>(68.0),
            Length::new::<millimeter>(135.0),
        )
    );
}

