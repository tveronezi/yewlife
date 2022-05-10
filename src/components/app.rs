use crate::components::actions::Actions;
use crate::components::existence::Existence;
use crate::components::msg_ctx::MessageProvider;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="h-screen">
            <MessageProvider>
                <Existence/>
                <Actions/>
            </MessageProvider>
        </div>
    }
}
