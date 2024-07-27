use std::ops::{Add, Div, Sub};

use uuid::Uuid;

use super::calculate_freezing_point;

#[derive(Debug, Clone)]
pub struct Drink {
    pub id: Uuid,
    pub name: String,
    pub description: String,
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
        description: &str,
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
        // let fluid_heat_transfer_coefficient = Fluid::Water.get_heat_transfer_coefficient();
        let fluid_heat_transfer_coefficient = 400.0;
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
            description: String::from(description),
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
            // Self::Aluminium => 237.,
            // Aluminum Alloy 3004 (UNS A93004)
            // https://www.azom.com/article.aspx?ArticleID=6619
            Self::Aluminium => 162.0,
            // Self::Aluminium => 10.0,
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
            // Fluid::Air => 40.,
            Fluid::Air => 25.,
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
