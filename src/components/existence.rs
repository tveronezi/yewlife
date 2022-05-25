use super::universe_ctx::Command;
use crate::components::bean::Bean;
use crate::components::universe_ctx::window_dimensions;
use crate::universe::Universe;
use gloo_events::EventListener;
use yew::prelude::*;

#[function_component(Existence)]
pub fn existence() -> Html {
    let universe = use_context::<UseReducerHandle<Universe>>().expect("no universe ctx found");
    let dimensions = use_state(window_dimensions);
    let dimensions_clone = dimensions.clone();
    let _ = use_state(|| {
        EventListener::new(&gloo_utils::window(), "resize", move |_| {
            let new_dyn = window_dimensions();
            dimensions_clone.set(new_dyn);
        })
    });
    let entities = universe
        .entities
        .iter()
        .filter(|e| (*e).is_visible(&dimensions))
        .map(|e| {
            html! {
                <Bean key={format!("c{}-l{}", e.column, e.line)} value={e.clone()} />
            }
        })
        .collect::<Html>();
    let onclick = Callback::from(move |e: MouseEvent| {
        let x = e.x();
        let y = e.y();
        universe.dispatch(Command::Add { x, y });
    });
    html! {
        <div {onclick} class="h-full w-full overflow-hidden relative">
          {entities}
        </div>
    }
}
