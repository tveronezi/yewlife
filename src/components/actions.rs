use crate::{
    components::icons::{GearIcon, PlayIcon, RefreshIcon, TrashIcon},
    universe::Universe,
};
use gloo_events::{EventListener, EventListenerOptions};
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::prelude::*;

use super::universe_ctx::Command;

#[derive(Debug)]
enum GearState {
    Expanded,
    Collapsed,
}

enum PlayState {
    Playing,
    Paused,
}

#[derive(PartialEq, Properties, Clone)]
pub struct ActionButtonProps {
    pub class: Option<String>,
    pub children: Children,
    pub onclick: Callback<()>,
    pub reference: Option<NodeRef>,
}

#[function_component(ActionButton)]
pub fn action_button(props: &ActionButtonProps) -> Html {
    let prop_callback = props.onclick.clone();
    let onclick = Callback::from(move |_| {
        prop_callback.emit(());
    });
    let class = &props.class;
    match &props.reference {
        Some(reference) => html! {
            <button ref={reference} {onclick} {class}>
                { for props.children.iter() }
            </button>
        },
        None => html! {
            <button {onclick} {class}>
                { for props.children.iter() }
            </button>
        },
    }
}

#[function_component(Actions)]
pub fn actions() -> Html {
    let universe = use_context::<UseReducerHandle<Universe>>().expect("no universe ctx found");
    let state = use_state(|| GearState::Collapsed);
    let cloned_state = state.clone();
    let on_gear_click = Callback::from(move |_| match *cloned_state {
        GearState::Expanded => cloned_state.set(GearState::Collapsed),
        GearState::Collapsed => cloned_state.set(GearState::Expanded),
    });
    let (clear_bnt, reset_btn) = match *state {
        GearState::Expanded => ("", ""),
        GearState::Collapsed => ("translate-y-[110px]", "translate-y-[60px]"),
    };
    let clear_btn_ref = use_node_ref();
    let reset_btn_ref = use_node_ref();
    let play_btn_ref = use_node_ref();
    let gear_btn_ref = use_node_ref();
    let interval: UseStateHandle<Option<Interval>> = use_state(|| None);
    let interval_clone = interval.clone();
    let universe_clone = universe.clone();
    let on_clear_click = Callback::from(move |_| {
        interval_clone.set(None);
        universe_clone.dispatch(Command::Clear);
    });
    let universe_clone = universe.clone();
    let interval_clone = interval.clone();
    let play_state = use_state(|| PlayState::Paused);
    let play_state_clone = play_state.clone();
    let on_play_click = Callback::from(move |_| {
        let universe_clone = universe_clone.clone();
        interval_clone.set(Some(Interval::new(100, move || {
            universe_clone.dispatch(Command::Tick);
        })));
        play_state_clone.set(PlayState::Playing);
    });
    let interval_clone = interval.clone();
    let play_state_clone = play_state.clone();
    let on_pause_click = Callback::from(move |_| {
        interval_clone.set(None);
        play_state_clone.set(PlayState::Paused);
    });
    let on_shuffle_click = Callback::from(move |_| {
        interval.set(None);
        universe.dispatch(Command::Shuffle);
    });
    let options = EventListenerOptions::run_in_capture_phase();
    let btns = vec![
        clear_btn_ref.clone(),
        reset_btn_ref.clone(),
        play_btn_ref.clone(),
        gear_btn_ref.clone(),
    ];
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
                        return;
                    }
                }
                state.set(GearState::Collapsed);
            },
        )
    });
    html! {
        <>
            <div class={format!("h-14 w-14 grid place-content-center fixed bottom-[145px] right-[85px] transition-transform {}", clear_bnt)}>
                <ActionButton reference={clear_btn_ref} onclick={on_clear_click} class="rounded-full bg-gray-500 p-2">
                    <TrashIcon svg_class="h-8 w-8 fill-gray-500" path_class="stroke-white"/>
                </ActionButton>
            </div>
            <div class={format!("h-14 w-14 grid place-content-center fixed bottom-[90px] right-[85px] transition-transform {}", reset_btn)}>
                <ActionButton reference={reset_btn_ref} onclick={on_shuffle_click} class="rounded-full bg-gray-500 p-2">
                    <RefreshIcon svg_class="h-8 w-8 fill-white"/>
                </ActionButton>
            </div>
            <div class="fixed bottom-[25px] right-[25px] flex items-center">
                <ActionButton reference={gear_btn_ref} onclick={on_gear_click} class="rounded-full bg-gray-500 p-1">
                    <GearIcon svg_class="h-14 w-14 fill-white"/>
                </ActionButton>
                {
                    match *play_state {
                        PlayState::Playing => html! {
                            <ActionButton onclick={on_pause_click}>
                                <PlayIcon svg_class="h-14 w-14 fill-yellow-600"/>
                            </ActionButton>
                        },
                        PlayState::Paused => html! {
                            <ActionButton reference={play_btn_ref} onclick={on_play_click}>
                                <PlayIcon svg_class="h-14 w-14 fill-yellow-600"/>
                            </ActionButton>
                        }
                    }
                }
            </div>
        </>
    }
}
