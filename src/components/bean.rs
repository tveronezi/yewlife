use std::rc::Rc;

use yew::prelude::*;

use crate::universe::{Entity, CELL_SIZE};

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub value: Rc<Entity>,
}

#[function_component(Bean)]
pub fn bean(props: &Props) -> Html {
    let (x, y) = (props.value.line * CELL_SIZE, props.value.column * CELL_SIZE);
    let style = format!(
        "height: {}px; width: {}px; top: {}px; left: {}px",
        CELL_SIZE, CELL_SIZE, x, y
    );
    html! {
        <div class="border border-black bg-cyan-800 absolute rounded-full" style={ style }/>
    }
}
