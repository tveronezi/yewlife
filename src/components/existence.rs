use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};
use crate::universe;

pub enum Msg {
    Tick,
}

pub struct Existence {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: universe::Universe,
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
        Self { link, value: universe }
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
        let universe = format!("{:?}", self.value);
        html! {
            <div class=classes!("app-center", "card-panel")>
                <a onclick=self.link.callback(|_| Msg::Tick) class="btn-floating btn-large waves-effect waves-light red"><i class="material-icons">{ "add" }</i></a>
                <p>
                    <span class="blue-text text-darken-2">{ universe }</span>
                </p>
            </div>
        }
    }
}
