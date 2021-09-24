use crate::universe;
use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    Tick,
}

pub struct Existence {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: universe::Universe,
}

trait Renderable {
    fn render(&self) -> Html;
}

impl Renderable for universe::Entity {
    fn render(&self) -> Html {
        html! {
            <div class=classes!("app-entity")>
            </div>
        }
    }
}

impl Component for Existence {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let universe = universe::Universe::new(
            r#"
010
001
111
        "#,
        );
        Self {
            link,
            value: universe,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                self.value.tick();
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        log::info!("{:?}", self.value);
        let mut entities = self
            .value
            .entities
            .iter()
            .map(|e| e.render())
            .collect::<Html>();
        html! {
            <div class=classes!("app-existence")>
                 { entities }
            </div>
        }
    }
}
