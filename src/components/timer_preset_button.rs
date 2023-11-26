use crate::helpers::TimerPreset;
use leptos::*;

#[component]
pub fn TimerPresetButton(
    preset: TimerPreset,
    modal_showing_signal: RwSignal<bool>,
    selected_preset_signal: RwSignal<TimerPreset>,
) -> impl IntoView {
    let preset_copy = preset.clone();
    view! {
        <div
            class="timer_preset_button button"
            on:click=move |_| {
                selected_preset_signal.set(preset.clone());
                modal_showing_signal.set(true);
            }
        >

            <img src=preset_copy.path_to_image/>
            <h4>{preset_copy.name}</h4>
        </div>
    }
}

