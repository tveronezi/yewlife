use yew::prelude::*;

use crate::universe::Entity;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub value: Entity,
}

#[function_component(Bean)]
pub fn bean(props: &Props) -> Html {
    let (x, y) = (
        props.value.line * crate::universe::CELL_SIZE,
        props.value.column * crate::universe::CELL_SIZE,
    );
    let style = format!(
        "height: {}px; width: {}px; top: {}px; left: {}px",
        crate::universe::CELL_SIZE,
        crate::universe::CELL_SIZE,
        x,
        y
    );
    html! {
        <div class="bg-blue-300 absolute" style={ style }/>
    }
}
