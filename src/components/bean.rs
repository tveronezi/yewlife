use yew::prelude::*;

use crate::universe::Entity;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub value: Entity,
}

#[function_component(Bean)]
pub fn bean(props: &Props) -> Html {
    html! {
        <div>{format!("{:?}", props.value)}</div>
    }
}
