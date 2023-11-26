use leptos::*;

use crate::{app::CurrentlyRunningTimers, format_chrono_duration_simple, TimerInfo};

#[component]
pub fn TimerTile(timer: TimerInfo) -> impl IntoView {
    let currently_running_timers = expect_context::<CurrentlyRunningTimers>().0;

    view! {
        <div class="timer_tile">
            <div class="temp_display">
                {move || timer.current_temperature.get().as_deg_celsius()} "°C"
            </div>
            <div class="time_display">
                {move || format_chrono_duration_simple(timer.current_time_left.get())}
            </div>
            <div class="controls">
                <button
                    class="cancel_timer_button"
                    on:click=move |_| {
                        currently_running_timers.update(|v| v.retain(|t| t.id != timer.id));
                    }
                >

                    "\u{1F5D9}"
                </button>
            </div>
        </div>
    }
}

