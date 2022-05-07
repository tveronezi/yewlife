use crate::components::existence::Existence;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App();

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="h-screen">
                <Existence/>
            </div>
        }
    }
}
