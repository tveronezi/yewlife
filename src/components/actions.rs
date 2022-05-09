use crate::components::icons::{GearIcon, PlayIcon, RefreshIcon, TrashIcon};
use yew::prelude::*;

#[function_component(Actions)]
pub fn actions() -> Html {
    html! {
        <>
            <div class="h-14 w-14 grid place-content-center fixed bottom-[145px] right-[85px]">
                <button class="rounded-full bg-gray-500 p-2">
                    <TrashIcon svg_class="h-8 w-8 fill-gray-500" path_class="stroke-white"/>
                </button>
            </div>
            <div class="h-14 w-14 grid place-content-center fixed bottom-[90px] right-[85px]">
                <button class="rounded-full bg-gray-500 p-2">
                    <RefreshIcon svg_class="h-8 w-8 fill-white"/>
                </button>
            </div>
            <div class="fixed bottom-[25px] right-[25px] flex items-center">
                <button class="rounded-full bg-gray-500 p-1">
                    <GearIcon svg_class="h-14 w-14 fill-white"/>
                </button>
                <button>
                    <PlayIcon svg_class="h-14 w-14 fill-yellow-600"/>
                </button>
            </div>
        </>
    }
}
