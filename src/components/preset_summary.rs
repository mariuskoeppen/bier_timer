use crate::{
    app::CurrentlyRunningTimers,
    helpers::{format_chrono_duration_simple, time_until_temperature, TimerPreset},
    TimerInfo,
};
use leptos::*;

#[component]
pub fn PresetSummary(
    preset_signal: RwSignal<TimerPreset>,
    modal_showing_signal: RwSignal<bool>,
) -> impl IntoView {
    let time_needed = Signal::derive(move || {
        time_until_temperature(
            preset_signal.get().target_ambience.temperature,
            preset_signal.get().initial_ambience.temperature,
            &preset_signal.get().drink,
            &preset_signal.get().ambient_ambience,
        )
    });

    let currently_running_timers = expect_context::<CurrentlyRunningTimers>().0;

    view! {
        <div class="preset_summary">
            <div class="summary">
                <div class="summary_sub first">
                    <div class="img_wrapper">
                        <img src=move || preset_signal.get().path_to_image/>
                    </div>
                    <span class="description">"Getränk"</span>
                    <span class="info">{move || preset_signal.get().name}</span>
                </div>
                <div class="summary_sub second">
                    <div class="img_wrapper">
                        {move || preset_signal.get().initial_ambience.temperature.as_deg_celsius()}
                        "° C"
                    </div>
                    <span class="description">"Ausgangstemperatur"</span>
                    <span class="info">{move || preset_signal.get().initial_ambience.name}</span>
                </div>
                <div class="summary_sub third">
                    <div class="img_wrapper">
                        {move || preset_signal.get().ambient_ambience.temperature.as_deg_celsius()}
                        "° C"
                    </div>
                    <span class="description">"Kühltemperatur"</span>
                    <span class="info">{move || preset_signal.get().ambient_ambience.name}</span>
                </div>
                <div class="summary_sub fourth">
                    <div class="img_wrapper">
                        {move || preset_signal.get().target_ambience.temperature.as_deg_celsius()}
                        "° C"
                    </div>
                    <span class="description">"Zieltemperatur"</span>
                    <span class="info">{move || preset_signal.get().target_ambience.name}</span>
                </div>
            </div>
            <div class="spacer"></div>
            <div class="time_display">

                {move || format_chrono_duration_simple(time_needed.get())}

            </div>
            <button
                class="start_timer_button button primary"
                on:click=move |_| {
                    let timer = TimerInfo::new(preset_signal.get());
                    currently_running_timers.update(move |v| v.push(timer));
                    modal_showing_signal.set(false);
                }
            >

                "Timer Starten"
            </button>
        </div>
    }
}

