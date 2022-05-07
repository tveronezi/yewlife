use crate::universe;
use yew::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Properties)]
pub struct Props {
    pub universe: Box<universe::Universe>,
}

pub struct Score {
    universe: Box<universe::Universe>,
}

impl Component for Score {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            universe: props.universe,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.universe != props.universe;
        self.universe = props.universe;
        changed
    }

    fn view(&self) -> Html {
        let elements = self.universe.entities.len();
        let visible = self
            .universe
            .entities
            .iter()
            .filter(|e| crate::components::existence::is_visible(*e))
            .count();
        html! {
            <div class="fixed top-4 right-4 bg-yellow-900/90 p-4 rounded-lg">
                <table class=classes!("table-auto")>
                    <tbody>
                    <tr>
                        <th class=classes!("text-white", "text-sm", "p-1")>{"Elements"}</th>
                        <td class=classes!("text-white", "text-xs", "p-1")>{ elements }</td>
                    </tr>
                    <tr>
                        <th class=classes!("text-white", "text-sm", "p-1")>{"Visible"}</th>
                        <td class=classes!("text-white", "text-xs", "p-1")>{ visible }</td>
                    </tr>
                    </tbody>
                </table>
            </div>
        }
    }
}
