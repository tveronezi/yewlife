use crate::universe;
use yew::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Properties)]
pub struct Props {
    pub universe: universe::Universe,
}

pub struct Score {
    universe: universe::Universe,
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
        let elements = self.universe.entities.iter().count();
        let visible = self
            .universe
            .entities
            .iter()
            .filter(|e| crate::components::existence::is_visible(*e))
            .count();
        html! {
            <div class=classes!("app-score")>
                <div class=classes!("card")>
                    <div class="card-content white-text">
                        <table>
                            <tbody>
                            <tr>
                                <th>{"Elements"}</th>
                                <td>{ elements }</td>
                            </tr>
                            <tr>
                                <th>{"Visible"}</th>
                                <td>{ visible }</td>
                            </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }
}
