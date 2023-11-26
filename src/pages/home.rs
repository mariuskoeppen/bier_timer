use crate::{
    app::CurrentlyRunningTimers,
    components::{PresetSummary, TimerPresetButton, TimerTile},
    TimerPreset,
};
use leptos::*;

#[component]
pub fn Home(
    timer_presets: Vec<TimerPreset>,
    selected_preset_signal: RwSignal<TimerPreset>,
) -> impl IntoView {
    let currently_running_timers = expect_context::<CurrentlyRunningTimers>().0;
    let modal_showing_signal = create_rw_signal(false);

    // let modal = create_node_ref::<Dialog>();
    // // modal.get().expect("to have modal").show();

    // create_effect(move |_| match modal_showing_signal.get() {
    //     true => {
    //         modal.get().expect("to have modal").show_modal();
    //     }
    //     false => {
    //         modal.get().expect("to have modal").close();
    //     }
    // });

    view! {
        <header>
            <h1>"Bier Timer"</h1>
        </header>
        <main>
            <section>
                <h3>"Laufende Timer"</h3>
                <div class="running_timers_wrapper">

                    {move || {
                        currently_running_timers
                            .get()
                            .iter()
                            .map(|timer_info| {
                                view! { <TimerTile timer=timer_info.clone()/> }
                            })
                            .collect_view()
                    }}

                </div>
            </section>

            <section>
                <h3>"Neuen Timer hinzufügen"</h3>

                <div class="timer_presets_wrapper">

                    {timer_presets
                        .iter()
                        .map(|preset| {
                            view! {
                                <TimerPresetButton
                                    preset=preset.clone()
                                    modal_showing_signal
                                    selected_preset_signal
                                />
                            }
                        })
                        .collect_view()}

                </div>
            </section>

            <section>
                <h3>"Weiterführende Informationen"</h3>
            </section>
        </main>
        <dialog id="preset_modal" class:hidden=move || !modal_showing_signal.get()>
            <div class="preset_modal_wrapper">
                <PresetSummary preset_signal=selected_preset_signal modal_showing_signal/>
            </div>
        </dialog>
    }
}

