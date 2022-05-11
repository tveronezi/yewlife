use crate::components::icons::{GearIcon, PlayIcon, RefreshIcon, TrashIcon};
use yew::prelude::*;

enum State {
    Expanded,
    Collapsed,
}

#[function_component(Actions)]
pub fn actions() -> Html {
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
    html! {
        <>
            <div class={format!("h-14 w-14 grid place-content-center fixed bottom-[145px] right-[85px] transition-transform {}", clear_bnt)}>
                <button class="rounded-full bg-gray-500 p-2">
                    <TrashIcon svg_class="h-8 w-8 fill-gray-500" path_class="stroke-white"/>
                </button>
            </div>
            <div class={format!("h-14 w-14 grid place-content-center fixed bottom-[90px] right-[85px] transition-transform {}", reset_btn)}>
                <button class="rounded-full bg-gray-500 p-2">
                    <RefreshIcon svg_class="h-8 w-8 fill-white"/>
                </button>
            </div>
            <div class="fixed bottom-[25px] right-[25px] flex items-center">
                <button onclick={on_gear_click} class="rounded-full bg-gray-500 p-1">
                    <GearIcon svg_class="h-14 w-14 fill-white"/>
                </button>
                <button>
                    <PlayIcon svg_class="h-14 w-14 fill-yellow-600"/>
                </button>
            </div>
        </>
    }
}
