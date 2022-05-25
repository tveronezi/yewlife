use std::rc::Rc;

use yew::prelude::*;

use crate::universe::{Entity, Universe, CELL_SIZE};

use super::universe_ctx::Command;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub value: Rc<Entity>,
}

#[function_component(Bean)]
pub fn bean(props: &Props) -> Html {
    let universe = use_context::<UseReducerHandle<Universe>>().expect("no universe ctx found");
    let (x, y) = (props.value.line * CELL_SIZE, props.value.column * CELL_SIZE);
    let style = format!(
        "height: {}px; width: {}px; top: {}px; left: {}px",
        CELL_SIZE, CELL_SIZE, x, y
    );
    let cloned_entity = props.value.clone();
    let onclick = Callback::from(move |_| {
        universe.dispatch(Command::Delete(cloned_entity.clone()));
    });
    html! {
        <div class="absolute" {style}>
            <div class="relative" style={format!("height: {}px; width: {}px;", CELL_SIZE, CELL_SIZE)}>
                <button {onclick} class="absolute w-full h-full border border-black bg-cyan-800 rounded-full" style={format!("height: {}px; width: {}px;", CELL_SIZE, CELL_SIZE)}/>
                <div class="transition-all bg-yellow-400 hover:bg-yellow-500 rounded-full blur-lg" style={format!("height: {}px; width: {}px;", CELL_SIZE, CELL_SIZE)}>
                </div>
            </div>
        </div>
    }
}
