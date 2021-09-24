use crate::components::bean::Bean;
use crate::universe;
use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};

#[derive(Debug)]
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
        log::info!("[create]: {:?}", universe);
        Self {
            link,
            value: universe,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("[update]: {:?}", msg);
        match msg {
            Msg::Tick => {
                self.value.tick();
                log::info!("[update]: re-render");
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        log::info!("[change]: {:?}", self.value);
        false
    }

    fn view(&self) -> Html {
        log::info!("[view]: {:?}", self.value);
        let entities = self
            .value
            .entities
            .iter()
            .map(|e| {
                html! {
                    <Bean value={e.clone()} />
                }
            })
            .collect::<Html>();
        html! {
            <div class=classes!("app-existence")>
                <div class="app-tick">
                    <a  onclick=self.link.callback(|_| Msg::Tick)
                        class="btn-floating btn-large waves-effect waves-light red"
                    >
                        <i class="material-icons">{ "add" }</i>
                    </a>
                </div>
                { entities }
            </div>
        }
    }
}
