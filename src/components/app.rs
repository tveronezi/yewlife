use crate::components::actions::Actions;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="h-screen">
            <Actions/>
        </div>
    }
}
