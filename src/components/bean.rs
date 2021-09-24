use crate::universe;
use yew::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    Die,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub value: universe::Entity,
}

pub struct Bean {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: universe::Entity,
}

impl Component for Bean {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: props.value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("[update]: {:?}", msg);
        match msg {
            Msg::Die => {
                // TODO: remove the element here
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.value != props.value;
        log::info!(
            "[change]: changed? {:?} [value: {:?}; props.value: {:?}]",
            changed,
            self.value,
            props.value
        );
        self.value = props.value;
        changed
    }

    fn view(&self) -> Html {
        log::info!("[view]: {:?}", self.value);
        let (x, y) = (self.value.line * 10, self.value.column * 10);
        let style = format!("top: {}px; left: {}px", x, y);
        let onclick = self.link.callback(|_| Msg::Die);
        html! {
            <div onclick={onclick} class=classes!("app-entity") style={ style }>
            </div>
        }
    }
}
