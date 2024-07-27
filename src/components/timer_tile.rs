use leptos::*;
use leptos_icons::{
    Icon,
    OcIcon::{OcCheckSm, OcXSm},
    VsIcon::VsCircleLargeFilled,
};

use crate::{
    app::CurrentlyRunningTimers, format_chrono_duration_precise, linear_interpolate_ceil,
    timer_info::TimerInfo,
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
    let timer_tile_expanded = create_rw_signal(false);

    view! {
        <button
            class="timer_tile"
            class:expanded=move || timer_tile_expanded.get()
            on:click=move |_ev| {
                timer_tile_expanded
                    .set(
                        match timer_tile_expanded.get() {
                            true => false,
                            false => true,
                        },
                    );
            }
        >

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

                {move || {
                    if timer.timer_finished.get() {
                        None
                    } else {
                        Some(
                            view! {
                                <span class="arrow">" -> "</span>
                                {timer
                                    .target_ambience
                                    .temperature
                                    .format(crate::TemperatureUnit::DegCelsius, true)}
                            },
                        )
                    }
                }}

            </div>
            <div class="time_display">
                {create_memo(move |_| format_chrono_duration_precise(
                    timer.current_time_left.get(),
                ))}

            </div>
            <div class="controls">
                <button
                    class="cancel_timer_button"
                    class:danger=move || !timer.timer_finished.get()
                    on:click=move |_| {
                        currently_running_timers.update(|v| v.retain(|t| t.id != timer.id));
                    }
                >

                    {create_memo(move |_| {
                        if !timer.timer_finished.get() {
                            view! { <Icon icon=Icon::from(OcXSm)/> }
                        } else {
                            view! { <Icon icon=Icon::from(OcCheckSm)/> }
                        }
                    })}

                </button>
            </div>

            <Show when=move || timer_tile_expanded.get() fallback=|| view! { "" }>
                <div class="more_info">
                    <p>
                        "Timer created: "
                        {move || timer.timestamp_started.naive_local().format("%H:%M").to_string()}
                    </p>

                    <p>
                        "Timer finishes: "
                        {move || timer.timestamp_finished.naive_local().format("%H:%M").to_string()}
                    </p>
                </div>
            </Show>
        </button>
    }
}

// "\u{1F5D9}"
