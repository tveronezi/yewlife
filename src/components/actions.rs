use crate::components::icons::{GearIcon, PlayIcon, RefreshIcon, TrashIcon};
use gloo_events::{EventListener, EventListenerOptions};
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub on_shuffle: Callback<()>,
    pub on_clear: Callback<()>,
    pub on_play: Callback<()>,
}

enum State {
    Expanded,
    Collapsed,
}

#[function_component(Actions)]
pub fn actions(props: &Props) -> Html {
    let state = use_state(|| State::Collapsed);
    let cloned_state = state.clone();
    let on_gear_click = Callback::from(move |_| match *cloned_state {
        State::Expanded => cloned_state.set(State::Collapsed),
        State::Collapsed => cloned_state.set(State::Expanded),
    });
    let (clear_bnt, reset_btn) = match *state {
        State::Expanded => ("", ""),
        State::Collapsed => ("translate-y-[110px]", "translate-y-[60px]"),
    };
    let clear_btn_ref = use_node_ref();
    let reset_btn_ref = use_node_ref();
    let play_btn_ref = use_node_ref();
    let gear_btn_ref = use_node_ref();
    let on_clear_clone = props.on_clear.clone();
    let on_clear_click = Callback::from(move |_| {
        on_clear_clone.emit(());
    });
    let on_play_clone = props.on_play.clone();
    let on_play_click = Callback::from(move |_| {
        on_play_clone.emit(());
    });
    let on_shuffle_clone = props.on_shuffle.clone();
    let on_shuffle_click = Callback::from(move |_| {
        on_shuffle_clone.emit(());
    });
    let options = EventListenerOptions::run_in_capture_phase();
    let btns = vec![
        clear_btn_ref.clone(),
        reset_btn_ref.clone(),
        play_btn_ref.clone(),
        gear_btn_ref.clone(),
    ];
    let cloned_state = state.clone();
    let _ = use_state(move || {
        EventListener::new_with_options(
            &gloo_utils::window(),
            "click",
            options,
            move |e: &Event| {
                e.cancel_bubble();
                let target = e
                    .target()
                    .expect("Event should have a target when dispatched");
                let target = target.dyn_into::<Node>().expect("Target should be a node");
                for btn in &btns {
                    let btn = btn.get().unwrap();
                    if btn.contains(Some(&target)) {
                        log::info!("clicked inside: {:?}", e.target());
                        return;
                    }
                }
                log::info!("clicked outside: {:?}", e.target());
                cloned_state.set(State::Collapsed);
            },
        )
    });
    html! {
        <>
            <div class={format!("h-14 w-14 grid place-content-center fixed bottom-[145px] right-[85px] transition-transform {}", clear_bnt)}>
                <button ref={clear_btn_ref} onclick={on_clear_click} class="rounded-full bg-gray-500 p-2">
                    <TrashIcon svg_class="h-8 w-8 fill-gray-500" path_class="stroke-white"/>
                </button>
            </div>
            <div class={format!("h-14 w-14 grid place-content-center fixed bottom-[90px] right-[85px] transition-transform {}", reset_btn)}>
                <button ref={reset_btn_ref} onclick={on_shuffle_click} class="rounded-full bg-gray-500 p-2">
                    <RefreshIcon svg_class="h-8 w-8 fill-white"/>
                </button>
            </div>
            <div class="fixed bottom-[25px] right-[25px] flex items-center">
                <button ref={gear_btn_ref} onclick={on_gear_click} class="rounded-full bg-gray-500 p-1">
                    <GearIcon svg_class="h-14 w-14 fill-white"/>
                </button>
                <button ref={play_btn_ref} onclick={on_play_click}>
                    <PlayIcon svg_class="h-14 w-14 fill-yellow-600"/>
                </button>
            </div>
        </>
    }
}
