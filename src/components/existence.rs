use crate::components::bean::Bean;
use crate::components::Dimensions;
use crate::universe;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub universe: universe::Universe,
    pub dimensions: Dimensions,
}

#[function_component(Existence)]
pub fn existence(props: &Props) -> Html {
    let entities = props
        .universe
        .entities
        .iter()
        .filter(|e| (*e).is_visible(&props.dimensions))
        .map(|e| {
            html! {
                <Bean key={format!("c{}-l{}", e.column, e.line)} value={e.clone()} />
            }
        })
        .collect::<Html>();
    html! {
        <div>
          {entities}
        </div>
    }
}
