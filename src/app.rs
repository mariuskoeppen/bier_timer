// use crate::calculate_surface_area::*;
use crate::helpers::*;
use crate::pages::*;
use ambience::Ambience;
use chrono::DateTime;
use chrono::{Duration, Local};
use drink::*;
use leptos::*;
use timer_info::TimerInfo;
use timer_preset::TimerPreset;

#[component]
pub fn App() -> impl IntoView {
    let drink_beer_5 = Drink::new(
        "Bier",
        "500ml Flasche",
        "./assets/images/bier5.svg",
        Container {
            volume: milliliters_to_m3(500.0),
            surface_area: 0.04064, // 0.04363705122848267 m^2
            material: ContainerMaterial::Glass,
            shape: ContainerShape::BeerBottle,
        },
        0.05,
    );
    let drink_beer_33 = Drink::new(
        "Bier",
        "330ml Flasche",
        "./assets/images/bier5.svg",
        Container {
            volume: milliliters_to_m3(330.0),
            surface_area: 0.03263,
            material: ContainerMaterial::Glass,
            shape: ContainerShape::BeerBottle,
        },
        0.05,
    );
    let drink_beer_5_can = Drink::new(
        "Bier",
        "500ml Dose",
        "./assets/images/can5.svg",
        Container {
            volume: milliliters_to_m3(500.0),
            surface_area: 0.03768, // 0.03825517374276292 m^2
            material: ContainerMaterial::Aluminium,
            shape: ContainerShape::Can,
        },
        0.05,
    );
    let drink_beer_33_can = Drink::new(
        "Bier",
        "330ml Dose",
        "./assets/images/can33.svg",
        Container {
            volume: milliliters_to_m3(330.0),
            surface_area: 0.02706,
            material: ContainerMaterial::Aluminium,
            shape: ContainerShape::Can,
        },
        0.05,
    );
    let drink_lemondade = Drink::new(
        "Limonade",
        "1L Flasche",
        "./assets/images/coke.svg",
        Container {
            volume: milliliters_to_m3(1000.0),
            surface_area: 0.06102,
            material: ContainerMaterial::Plastic,
            shape: ContainerShape::PetBottle,
        },
        0.00,
    );
    let drink_wine = Drink::new(
        "Rotwein",
        "750ml Flasche",
        "./assets/images/wein_rot.svg",
        Container {
            volume: milliliters_to_m3(750.0),
            surface_area: 0.05138,
            material: ContainerMaterial::Glass,
            shape: ContainerShape::WineBottle,
        },
        0.15,
    );
    let drink_liquor = Drink::new(
        "Schnaps",
        "700ml Flasche",
        "./assets/images/vodka.svg",
        Container {
            volume: milliliters_to_m3(700.0),
            surface_area: 0.04844,
            material: ContainerMaterial::Glass,
            shape: ContainerShape::SchnapsBottle,
        },
        0.40,
    );

    // Initial Temperatures
    let initial_kellerkalt = Ambience::new(
        "Kellerkalt",
        "./assets/images/ioicon/thermometer-outline.svg",
        Temperature::new_with_unit(14.0, TemperatureUnit::DegCelsius),
        None,
    );
    let initial_raumtemperatur = Ambience::new(
        "Raumtemperatur",
        "./assets/images/ioicon/partly-sunny-outline.svg",
        Temperature::new_with_unit(20.0, TemperatureUnit::DegCelsius),
        None,
    );
    let initial_sommertag = Ambience::new(
        "Heißer Sommertag",
        "./assets/images/ioicon/sunny-outline.svg",
        Temperature::new_with_unit(30.0, TemperatureUnit::DegCelsius),
        None,
    );

    // Ambient Temperatures
    let ambient_eisfach = Ambience::new(
        "Eisfach",
        "./assets/images/flake3.svg",
        Temperature::new_with_unit(-18.0, TemperatureUnit::DegCelsius),
        Some(Fluid::Air),
    );
    let ambient_eisbad = Ambience::new(
        "Eisbad",
        "./assets/images/flake.svg",
        Temperature::new_with_unit(0.0, TemperatureUnit::DegCelsius),
        Some(Fluid::Water),
    );
    let ambient_kuehlschrank = Ambience::new(
        "Kühlschrank",
        "./assets/images/ioicon/thermometer-outline.svg",
        Temperature::new_with_unit(5.0, TemperatureUnit::DegCelsius),
        Some(Fluid::Air),
    );

    // Target Temperatures
    let target_schnaps = Ambience::new(
        "Optimal für Schnaps",
        "./assets/images/vodka.svg",
        Temperature::new_with_unit(2.0, TemperatureUnit::DegCelsius),
        None,
    );
    let target_lemonade = Ambience::new(
        "Optimal für Limonade",
        "./assets/images/coke.svg",
        Temperature::new_with_unit(4.0, TemperatureUnit::DegCelsius),
        None,
    );
    let target_beer = Ambience::new(
        "Optimal für Bier",
        "./assets/images/bier5.svg",
        Temperature::new_with_unit(6.0, TemperatureUnit::DegCelsius),
        None,
    );
    let target_wine_white = Ambience::new(
        "Optimal für Weißwein",
        "./assets/images/wein_weiss.svg",
        Temperature::new_with_unit(10.0, TemperatureUnit::DegCelsius),
        None,
    );
    let target_wine_red = Ambience::new(
        "Optimal für Rotwein",
        "./assets/images/wein_rot.svg",
        Temperature::new_with_unit(16.0, TemperatureUnit::DegCelsius),
        None,
    );

    // Timer Presets
    let preset_beer = TimerPreset {
        name: "Bier".to_string(),
        path_to_image: target_beer.path_to_image.clone(),
        drink: drink_beer_5.clone(),
        initial_ambience: initial_raumtemperatur.clone(),
        ambient_ambience: ambient_eisfach.clone(),
        target_ambience: target_beer.clone(),
    };
    let preset_wine_red = TimerPreset {
        name: "Rotwein".to_string(),
        path_to_image: target_wine_red.path_to_image.clone(),
        drink: drink_wine.clone(),
        initial_ambience: initial_raumtemperatur.clone(),
        ambient_ambience: ambient_eisfach.clone(),
        target_ambience: target_wine_red.clone(),
    };
    let preset_wine_white = TimerPreset {
        name: "Weißwein".to_string(),
        path_to_image: target_wine_white.path_to_image.clone(),
        drink: drink_wine.clone(),
        initial_ambience: initial_raumtemperatur.clone(),
        ambient_ambience: ambient_eisfach.clone(),
        target_ambience: target_wine_white.clone(),
    };
    let preset_schnaps = TimerPreset {
        name: "Schnaps".to_string(),
        path_to_image: target_schnaps.path_to_image.clone(),
        drink: drink_liquor.clone(),
        initial_ambience: initial_raumtemperatur.clone(),
        ambient_ambience: ambient_eisfach.clone(),
        target_ambience: target_schnaps.clone(),
    };
    let preset_beer_can = TimerPreset {
        name: "Bier Dose 500".to_string(),
        path_to_image: target_beer.path_to_image.clone(),
        drink: drink_beer_5_can.clone(),
        initial_ambience: initial_raumtemperatur.clone(),
        ambient_ambience: ambient_eisfach.clone(),
        target_ambience: target_beer.clone(),
    };
    let preset_dev = TimerPreset {
        name: "Dev".to_string(),
        path_to_image: target_beer.path_to_image.clone(),
        drink: drink_beer_5_can.clone(),
        initial_ambience: Ambience::new(
            "dev1",
            "./assets/images/wein_rot.svg",
            Temperature::new_with_unit(11.0, TemperatureUnit::DegCelsius),
            None,
        )
        .clone(),
        ambient_ambience: Ambience::new(
            "dev2",
            "./assets/images/wein_rot.svg",
            Temperature::new_with_unit(-50.0, TemperatureUnit::DegCelsius),
            None,
        )
        .clone(),
        target_ambience: Ambience::new(
            "dev3",
            "./assets/images/wein_rot.svg",
            Temperature::new_with_unit(10.0, TemperatureUnit::DegCelsius),
            None,
        )
        .clone(),
    };

    let timer_presets = vec![
        preset_beer.clone(),
        preset_wine_red.clone(),
        preset_wine_white.clone(),
        preset_schnaps.clone(),
        preset_beer_can.clone(),
        preset_dev.clone(),
    ];
    let selected_preset_signal = create_rw_signal(preset_beer);

    // leptos::logging::log!("{:#?}", drinks);

    // Current Time
    let current_time_signal = create_rw_signal(Local::now());
    set_interval(
        move || {
            current_time_signal.set(Local::now());
        },
        Duration::milliseconds(997) // Every roughly 1/4 second, prime number
            .to_std()
            .expect("to convert to std::time::Duration"),
    );
    provide_context(CurrentTime(current_time_signal));

    // Running timers
    let currently_running_timers = create_rw_signal::<Vec<TimerInfo>>(vec![]);
    provide_context(CurrentlyRunningTimers(currently_running_timers));

    view! { <Home timer_presets=timer_presets.clone() selected_preset_signal/> }
}

#[derive(Clone)]
pub struct CurrentTime(pub RwSignal<DateTime<Local>>);

#[derive(Clone)]
pub struct CurrentlyRunningTimers(pub RwSignal<Vec<TimerInfo>>);
