use crate::{
    components::icons::{GearIcon, PauseIcon, PlayIcon, RefreshIcon, TrashIcon},
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
    let content = html! {
        <div class="relative">
            <div class="w-10 h-10 absolute">
                { for props.children.iter() }
            </div>
            <div class="w-10 h-10 transition-all hover:bg-yellow-500 rounded-full blur-lg">
            </div>
        </div>
    };
    let class = &props.class;
    match &props.reference {
        Some(reference) => html! {
            <button ref={reference} {onclick} {class}>
                { content }
            </button>
        },
        None => html! {
            <button {onclick} {class}>
                { content }
            </button>
        },
    }
}

#[function_component(Actions)]
pub fn actions() -> Html {
    let universe = use_context::<UseReducerHandle<Universe>>().expect("no universe ctx found");
    let gear_state = use_state(|| GearState::Collapsed);
    let cloned_gear_state = gear_state.clone();
    let on_gear_click = Callback::from(move |_| match *cloned_gear_state {
        GearState::Expanded => cloned_gear_state.set(GearState::Collapsed),
        GearState::Collapsed => cloned_gear_state.set(GearState::Expanded),
    });
    let clear_btn_ref = use_node_ref();
    let reset_btn_ref = use_node_ref();
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
        gear_btn_ref.clone(),
    ];
    let gear_actions_cls = match *gear_state {
        GearState::Expanded => "",
        GearState::Collapsed => "translate-y-0 opacity-0",
    };
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
                gear_state.set(GearState::Collapsed);
            },
        )
    });
    html! {
        <div class="flex fixed bottom-0 right-0 p-2">
            <div class="space-y-2">
                <div class="relative">
                    <ActionButton reference={clear_btn_ref} onclick={on_clear_click} class={format!("grid place-items-center absolute -translate-y-[90px] opacity-100 transition-all {}", gear_actions_cls)}>
                        <TrashIcon class="h-10 w-10 fill-gray-400"/>
                    </ActionButton>
                    <ActionButton reference={reset_btn_ref} onclick={on_shuffle_click} class={format!("grid place-items-center absolute -translate-y-[40px] opacity-100 transition-all {}", gear_actions_cls)}>
                        <RefreshIcon class="h-10 w-10 fill-gray-400"/>
                    </ActionButton>
                </div>
                <ActionButton reference={gear_btn_ref} onclick={on_gear_click} class="grid place-items-center">
                    <GearIcon class="h-10 w-10 fill-white"/>
                </ActionButton>
            </div>
            <div class="flex items-end">
            {
                match *play_state {
                    PlayState::Playing => html! {
                        <ActionButton onclick={on_pause_click}>
                            <PauseIcon class="h-10 w-10 fill-green-400"/>
                        </ActionButton>
                    },
                    PlayState::Paused => html! {
                        <ActionButton onclick={on_play_click}>
                            <PlayIcon class="h-10 w-10 fill-yellow-400"/>
                        </ActionButton>
                    }
                }
            }
            </div>
        </div>
    }
}
