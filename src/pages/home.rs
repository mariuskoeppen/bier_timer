use crate::{
    app::CurrentlyRunningTimers,
    components::{PresetSummary, TimerPresetButton, TimerTile},
    TimerPreset,
};
use leptos::*;
use leptos_icons::{Icon, OcIcon::OcXSm};

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
        <div class="main_content_container">
            <header>
                <h1>"Bier Timer"</h1>
            </header>
            <main>
                <section>
                    <h3>"Laufende Timer"</h3>
                    <div class="running_timers_wrapper">

                        {move || {
                            if currently_running_timers.get().len() == 0 {
                                view! { <p>"Erstelle unten einen neuen Timer"</p> }.into_view()
                            } else {
                                currently_running_timers
                                    .get()
                                    .iter()
                                    .map(|timer_info| {
                                        view! { <TimerTile timer=timer_info.clone()/> }
                                    })
                                    .collect_view()
                            }
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

            // <section>
            // <h3>"Weiterführende Informationen"</h3>
            // </section>
            </main>
        </div>
        <div
            id="preset_modal"
            class:hidden=move || !modal_showing_signal.get()
            on:click:undelegated=move |e| {
                if e.target().expect("click target")
                    != e.current_target().expect("click current target")
                {
                    return;
                }
                modal_showing_signal.set(false);
            }
        >

            <div class="preset_modal_wrapper">
                <div class="close_button button" on:click=move |_| modal_showing_signal.set(false)>
                    <Icon icon=Icon::from(OcXSm)/>
                </div>
                <PresetSummary preset_signal=selected_preset_signal modal_showing_signal/>
            </div>
        </div>
    }
}
