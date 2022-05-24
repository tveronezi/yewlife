use crate::components::actions::Actions;
use crate::components::existence::Existence;
use crate::components::universe_ctx::UniverseProvider;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="h-screen bg-black">
            <UniverseProvider>
                <Existence />
                <Actions />
            </UniverseProvider>
        </div>
    }
}
