use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    AddOne,
}

pub struct Counter {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
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
        html! {
            <div class=classes!("app-center", "card-panel")>
                <a onclick=self.link.callback(|_| Msg::AddOne) class="btn-floating btn-large waves-effect waves-light red"><i class="material-icons">{ "add" }</i></a>
                <p>
                    <span class="blue-text text-darken-2">{ self.value }</span>
                </p>
            </div>
        }
    }
}
