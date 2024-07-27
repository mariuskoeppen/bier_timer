use chrono::{DateTime, Duration, Local};
use leptos::*;
use uuid::Uuid;

use super::{
    ambience::{self, Ambience},
    temperature_after_time, time_until_temperature,
    timer_preset::TimerPreset,
    Drink, Temperature,
};

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
    pub timer_finished: Signal<bool>,
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
        let current_time_left = Signal::derive(move || finished - current_time_signal.get());
        let timer_finished = Signal::derive(move || current_time_left.get() < Duration::zero());
        TimerInfo {
            id: Uuid::new_v4(),
            timestamp_started: start,
            timestamp_finished: finished,
            drink: preset.drink.clone(),
            initial_ambience: preset.initial_ambience.clone(),
            ambient_ambience: preset.ambient_ambience.clone(),
            target_ambience: preset.target_ambience,
            current_time_left,
            current_temperature: Signal::derive(move || {
                temperature_after_time(
                    current_time_signal.get() - start,
                    preset.initial_ambience.temperature,
                    &preset.drink,
                    &preset.ambient_ambience,
                )
            }),
            timer_finished,
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
