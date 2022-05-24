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
        <button {onclick} class="border border-black bg-cyan-800 absolute rounded-full" style={ style }/>
    }
}
