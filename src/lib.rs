mod app;
mod components;
mod pages;

pub use self::{app::App, helpers::*, pages::*};

pub mod helpers {
    #![allow(unused)]
    use chrono::{DateTime, Duration, Local};
    use leptos::{create_rw_signal, RwSignal, Signal, SignalGet, SignalUpdate};
    use std::ops::{Add, Div, Sub};
    use uuid::Uuid;

    #[derive(Clone)]
    pub struct TimerInfo {
        // frozen
        // timed out?
        pub id: Uuid,
        pub timestamp_started: DateTime<Local>,
        pub timestamp_finished: DateTime<Local>,
        pub drink: Drink,
        pub initial_ambience: Ambience,
        pub ambient_ambience: Ambience,
        pub target_ambience: Ambience,
        pub current_time_left: Signal<Duration>,
        pub current_temperature: Signal<Temperature>,
    }

    impl TimerInfo {
        pub fn new(preset: TimerPreset, current_time_signal: RwSignal<DateTime<Local>>) -> Self {
            let start = Local::now();
            let finished = start
                + time_until_temperature(
                    preset.target_ambience.temperature,
                    preset.initial_ambience.temperature,
                    &preset.drink,
                    &preset.ambient_ambience,
                );
            TimerInfo {
                id: Uuid::new_v4(),
                timestamp_started: start,
                timestamp_finished: finished,
                drink: preset.drink.clone(),
                initial_ambience: preset.initial_ambience.clone(),
                ambient_ambience: preset.ambient_ambience.clone(),
                target_ambience: preset.target_ambience,
                current_time_left: Signal::derive(move || finished - current_time_signal.get()),
                current_temperature: Signal::derive(move || {
                    temperature_after_time(
                        current_time_signal.get() - start,
                        preset.initial_ambience.temperature,
                        &preset.drink,
                        &preset.ambient_ambience,
                    )
                }),
            }
        }

        // pub fn update(&self, current_time: DateTime<Local>) {
        //     self.current_time_left
        //         .update(|time| *time = self.timestamp_finished - current_time);
        //     let time_passed = current_time - self.timestamp_started;
        //     self.current_temperature.update(|temperature| {
        //         *temperature = temperature_after_time(
        //             time_passed,
        //             self.initial_ambience.temperature,
        //             &self.drink,
        //             &self.ambient_ambience,
        //         )
        //     });
        // }
    }

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

        let gradient = (target_temperature - ambience.temperature)
            / (initial_temperature - ambience.temperature);

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

    #[derive(Debug, Clone)]
    pub struct Drink {
        pub id: Uuid,
        pub name: String,
        pub path_to_image: String,
        pub container: Container,
        /// Value between 0. and 1., volumetric percentage of
        /// ethanol in fluid.
        pub alcohol_percentage: f64,
        // sugar_percentage: f64,
        // pub drink_type: DrinkType,
        // pub heat_capacity: f64,
        // pub fluid_heat_transfer_coefficient: f64,
        // pub container_heat_transfer_coefficient: f64,
        /// First value is for ambient air (freezer, fridge),
        /// Second value for ambient water (ice bath)
        pub cooling_coefficient: (f64, f64),
        pub freezing_point: Temperature,
    }

    impl Drink {
        pub fn new(
            name: &str,
            path_to_image: &str,
            container: Container,
            alcohol_percentage: f64,
        ) -> Self {
            // Density -- kg / m^3
            let water_density = Fluid::Water.get_density();
            let ethanol_density = Fluid::Ethanol.get_density();

            // Volume -- m^3
            let total_volume = container.volume;
            let water_volume_percentage = 1. - alcohol_percentage;
            let water_volume = total_volume * water_volume_percentage;
            let ethanol_volume = total_volume * alcohol_percentage;

            // Mass -- kg
            let water_mass = water_density * water_volume;
            let ethanol_mass = ethanol_density * ethanol_volume;
            let total_mass = water_mass + ethanol_mass;

            // Heat capacity of two mixed fluids
            // (https://thermtest.com/thermal-resources/rule-of-mixtures)
            // Heat capacity -- J / (kg * K)
            let water_heat_capacity = Fluid::Water.get_heat_capacity();
            let ethanol_heat_capacity = Fluid::Ethanol.get_heat_capacity();

            let total_heat_capacity_coefficient = water_heat_capacity * (water_mass / total_mass)
                + ethanol_heat_capacity * (ethanol_mass / total_mass);

            let total_heat_capacity = total_mass * total_heat_capacity_coefficient;

            // Heat transfer coefficient -- W / (m^2 * K)
            // Todo: calculate htc based on ethanol content
            let fluid_heat_transfer_coefficient = Fluid::Water.get_heat_transfer_coefficient();
            let container_heat_transfer_coefficient = container.get_heat_transfer_coefficient();

            // Todo: Can I precalculate the area based on volume etc.?
            //

            let total_heat_transfer_coefficient_ambient_air = 1.
                / (1. / Fluid::Air.get_heat_transfer_coefficient()
                    + 1. / container_heat_transfer_coefficient
                    + 1. / fluid_heat_transfer_coefficient);
            let total_heat_transfer_coefficient_ambient_water = 1.
                / (1. / Fluid::Water.get_heat_transfer_coefficient()
                    + 1. / container_heat_transfer_coefficient
                    + 1. / fluid_heat_transfer_coefficient);

            let cooling_coefficient_air = total_heat_transfer_coefficient_ambient_air
                * container.surface_area
                / total_heat_capacity;
            let cooling_coefficient_water = total_heat_transfer_coefficient_ambient_water
                * container.surface_area
                / total_heat_capacity;

            let cooling_coefficient = (cooling_coefficient_air, cooling_coefficient_water);

            let freezing_point = calculate_freezing_point(alcohol_percentage);

            Drink {
                id: Uuid::new_v4(),
                name: String::from(name),
                path_to_image: String::from(path_to_image),
                container: container.clone(),
                alcohol_percentage,
                cooling_coefficient,
                freezing_point,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct Container {
        /// Volume in m^3
        pub volume: f64,
        // /// Thickness of the wall in m
        // pub wall_thickness: f64,
        /// Surface area of the container in m^2
        pub surface_area: f64,
        pub material: ContainerMaterial,
        pub shape: ContainerShape,
    }

    impl HeatTransferCoefficient for Container {
        fn get_heat_transfer_coefficient(&self) -> f64 {
            self.material.get_heat_transfer_coefficient()
        }
    }

    impl ThermalConductivity for Container {
        fn get_thermal_conductivity(&self) -> f64 {
            self.material.get_thermal_conductivity()
        }
    }

    #[derive(Debug, Clone, Default)]
    pub enum ContainerMaterial {
        Plastic,
        #[default]
        Glass,
        Aluminium,
    }

    impl ContainerMaterial {
        /// Thickness of wall of bottle in meters.
        ///
        /// The thicker the wall, the longer it takes for
        /// heat/power to travel through the material.
        fn get_thickness(&self) -> f64 {
            match self {
                Self::Plastic => 0.001,
                Self::Glass => 0.0045,
                Self::Aluminium => 0.001,
            }
        }
    }

    impl ThermalConductivity for ContainerMaterial {
        fn get_thermal_conductivity(&self) -> f64 {
            match self {
                Self::Plastic => 0.01,
                Self::Glass => 0.037,
                Self::Aluminium => 237.,
            }
        }
    }

    impl HeatTransferCoefficient for ContainerMaterial {
        fn get_heat_transfer_coefficient(&self) -> f64 {
            self.get_thermal_conductivity() / self.get_thickness()
        }
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    /// Mainly for classification
    pub enum DrinkType {
        #[default]
        Beer,
        Wine,
        Lemonade,
        Schnaps,
        Other,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub enum ContainerShape {
        #[default]
        BeerBottle,
        WineBottle,
        /// Beer or soda can
        Can,
        /// Plastic bottle
        PetBottle,
        SchnapsBottle,
    }

    #[derive(Debug, Clone)]
    pub enum Fluid {
        Air,
        Water,
        Ethanol,
    }

    impl Fluid {
        /// Density in kg / m^3
        pub fn get_density(&self) -> f64 {
            match self {
                Fluid::Water => 1000.,
                Fluid::Ethanol => 789.,
                _ => todo!("air density not needed"),
            }
        }

        /// In J / (kg * K)
        pub fn get_heat_capacity(&self) -> f64 {
            match self {
                Fluid::Water => 4182.,
                Fluid::Ethanol => 2460.,
                // Fluid::Ethanol => 3460.,
                _ => todo!("air capacity not needed"),
            }
        }
    }

    impl HeatTransferCoefficient for Fluid {
        fn get_heat_transfer_coefficient(&self) -> f64 {
            match self {
                Fluid::Air => 40.,
                Fluid::Water => 1000.,
                _ => todo!("other fluids not implemented"),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Temperature(f64);

    impl Temperature {
        pub fn new(temperature_kelvin: f64) -> Self {
            Temperature::new_with_unit(temperature_kelvin, TemperatureUnit::Kelvin)
        }

        pub fn new_with_unit(value: f64, unit: TemperatureUnit) -> Self {
            match unit {
                TemperatureUnit::Kelvin => Temperature(value),
                TemperatureUnit::DegCelsius => Temperature(value + 273.15),
            }
        }

        pub fn as_unit(&self, unit: TemperatureUnit) -> f64 {
            match unit {
                TemperatureUnit::Kelvin => self.0,
                TemperatureUnit::DegCelsius => self.0 - 273.15,
            }
        }

        pub fn as_kelvin(&self) -> f64 {
            self.as_unit(TemperatureUnit::Kelvin)
        }

        pub fn as_deg_celsius(&self) -> f64 {
            self.as_unit(TemperatureUnit::DegCelsius)
        }

        pub fn format(&self, unit: TemperatureUnit, append_unit: bool) -> String {
            let raw = match unit {
                TemperatureUnit::DegCelsius => self.as_deg_celsius(),
                TemperatureUnit::Kelvin => self.as_kelvin(),
            };

            let append = match unit {
                TemperatureUnit::DegCelsius => " °C",
                TemperatureUnit::Kelvin => " K",
            };

            format!("{:.0}{}", raw, append)
        }
    }

    impl Add for Temperature {
        type Output = Temperature;

        fn add(self, rhs: Self) -> Self::Output {
            Temperature::new(self.0 + rhs.0)
        }
    }

    impl Sub for Temperature {
        type Output = Temperature;

        fn sub(self, rhs: Self) -> Self::Output {
            Temperature::new(self.0 - rhs.0)
        }
    }

    impl Div for Temperature {
        type Output = f64;

        fn div(self, rhs: Self) -> Self::Output {
            self.0 / rhs.0
        }
    }

    #[derive(Debug, Clone)]
    pub enum TemperatureUnit {
        Kelvin,
        DegCelsius,
        // DegFahrenheit,
    }

    /// Heat transfer coefficient in W / (m^2 * K)
    ///
    /// Describes how much heat-flow can pass through an area of this
    /// material.
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Heat_transfer_coefficient)
    pub trait HeatTransferCoefficient {
        /// Heat transfer coefficient in W / (m^2 * K)
        fn get_heat_transfer_coefficient(&self) -> f64;
    }

    /// Thermal conductivity in W / (m * K).
    ///
    /// Property to describe heat throughput: Higher means
    /// more heat/power can travel through the material.
    ///
    /// See [Wikipedia](https://en.wikipedia.org/wiki/Thermal_conductivity_and_resistivity)
    pub trait ThermalConductivity {
        /// Thermal conductivity in W / (m * K).
        fn get_thermal_conductivity(&self) -> f64;
    }

    // pub trait CoolingCoefficient {
    //     /// Cooling coefficient in 1 / s
    //     /// https://en.wikipedia.org/wiki/Newton%27s_law_of_cooling
    //     fn get_cooling_coefficient(&self) -> f64;
    // }

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
            return format!("0:00:00");
        }
        let seconds = duration.num_seconds() % 60;
        let minutes = duration.num_minutes() % 60;
        let hours = duration.num_hours() % 24;

        if hours < 1 {
            if minutes < 1 {
                return format!("{:0>1}", seconds);
            }

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
}

