#![allow(unused)]
pub mod ambience;
pub mod drink;
pub mod timer_info;
pub mod timer_preset;

pub use ambience::Ambience;
use chrono::Duration;
pub use drink::{Drink, Fluid, Temperature, TemperatureUnit};
pub use timer_preset::TimerPreset;

/// How long does it take until a drink reaches its
/// target temperature
///
/// Reverse function of [`temperature_after_time`]
pub fn time_until_temperature(
    target_temperature: Temperature,
    initial_temperature: Temperature,
    drink: &Drink,
    ambience: &Ambience,
) -> Duration {
    let cooling_coefficient = match ambience.fluid {
        Fluid::Air => drink.cooling_coefficient.0,
        Fluid::Water => drink.cooling_coefficient.1,
        _ => todo!("no cooling coefficient for other fluids than water and air"),
    };

    let gradient =
        (target_temperature - ambience.temperature) / (initial_temperature - ambience.temperature);

    let t = -f64::log(gradient, std::f64::consts::E) / cooling_coefficient;

    Duration::milliseconds(t as i64 * 1000)
}

/// Calculates the current temperature the drink has
/// when it has been in the freezer/fridge for specified time
///
/// Reverse function of [`time_until_temperature`]
pub fn temperature_after_time(
    time: Duration,
    initial_temperature: Temperature,
    drink: &Drink,
    ambience: &Ambience,
) -> Temperature {
    let cooling_coefficient = match ambience.fluid {
        Fluid::Air => drink.cooling_coefficient.0,
        Fluid::Water => drink.cooling_coefficient.1,
        _ => todo!("no cooling coefficient for other fluids than water and air"),
    };

    let time = time.num_milliseconds() as f64 / 1000.;
    let temperature = ambience.temperature.as_kelvin()
        + (initial_temperature.as_kelvin() - ambience.temperature.as_kelvin())
            * f64::exp(-cooling_coefficient * time);

    Temperature::new(temperature)
}

fn calculate_freezing_point(alcohol: f64) -> Temperature {
    // https://www.engineeringtoolbox.com/ethanol-water-d_989.html
    let t = linear_interpolate(alcohol, 0.0, 0.6, 0.0, -37.0);

    Temperature::new_with_unit(t, TemperatureUnit::DegCelsius)
}

pub fn linear_interpolate_ceil(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    linear_interpolate(x.clamp(x0.min(x1), x0.max(x1)), x0, x1, y0, y1)
}

pub fn linear_interpolate(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
}

pub fn milliliters_to_m3(milliliters: f64) -> f64 {
    milliliters / 1_000_000.
}

pub fn format_chrono_duration(duration: Duration) -> String {
    if duration <= Duration::zero() {
        return format!("0:00:00");
    }
    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours() % 24;

    if hours < 1 {
        return format!("{:0>2} mins", minutes);
    }

    format!("{:0>1}:{:0>2} hrs", hours, minutes)
}

pub fn format_chrono_duration_precise(duration: Duration) -> String {
    if duration <= Duration::zero() {
        return format!("0:00");
    }
    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours() % 24;

    if hours < 1 {
        return format!("{:0>1}:{:0>2}", minutes, seconds);
    }

    format!("{:0>1}:{:0>2}:{:0>2}", hours, minutes, seconds)
}

pub fn format_chrono_duration_simple(duration: Duration) -> String {
    if duration <= Duration::zero() {
        return format!("0:00:00");
    }
    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours() % 24;

    format!("{:0>1}:{:0>2}", hours, minutes)
}
