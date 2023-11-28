use leptos::*;
use leptos_icons::{Icon, OcIcon::OcXSm, VsIcon::VsCircleLargeFilled};

use crate::{
    app::CurrentlyRunningTimers, format_chrono_duration_precise, linear_interpolate_ceil, TimerInfo,
};

#[component]
pub fn TimerTile(timer: TimerInfo) -> impl IntoView {
    let currently_running_timers = expect_context::<CurrentlyRunningTimers>().0;
    let circle_color = create_memo(move |_| {
        format!(
            "color-mix(in srgb, var(--color-hot), var(--color-cold) {}%)",
            linear_interpolate_ceil(
                timer.current_temperature.get().as_deg_celsius(),
                timer.initial_ambience.temperature.as_deg_celsius(),
                timer.target_ambience.temperature.as_deg_celsius(),
                0.0,
                100.0,
            )
            .round(),
        )
    });
    view! {
        <div class="timer_tile">
            <div class="temp_display">

                <style>
                    "
                    .temp_circle-id-" {timer.id.to_string()} "{" "--heat-color: " {circle_color} "}
                    
                    // @property --heat-color-property {
                    //     syntax: '<color>';
                    //     initial-value: " {circle_color} ";
                    //     inherits: false;
                    // }
                    
                    "
                </style>
                <Icon
                    class=format!("temp_circle temp_circle-id-{}", timer.id.to_string())
                    icon=Icon::from(VsCircleLargeFilled)
                />
                // style=

                {create_memo(move |_| {
                    timer.current_temperature.get().format(crate::TemperatureUnit::DegCelsius, true)
                })}

                <span class="arrow">" / "</span>
                {timer.target_ambience.temperature.format(crate::TemperatureUnit::DegCelsius, true)}

            </div>
            <div class="time_display">
                {create_memo(move |_| format_chrono_duration_precise(
                    timer.current_time_left.get(),
                ))}

            </div>
            <div class="controls">
                <button
                    class="cancel_timer_button danger"
                    on:click=move |_| {
                        currently_running_timers.update(|v| v.retain(|t| t.id != timer.id));
                    }
                >

                    <Icon icon=Icon::from(OcXSm)/>

                </button>
            </div>
        </div>
    }
}

// "\u{1F5D9}"

