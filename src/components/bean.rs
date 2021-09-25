use crate::universe;
use yew::{classes, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    Die,
}

#[derive(Debug)]
pub enum CallbackMsg {
    Die(universe::Entity),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub value: universe::Entity,
    pub onchange: Callback<CallbackMsg>,
}

pub struct Bean {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: universe::Entity,
    onchange: Callback<CallbackMsg>,
}

impl Component for Bean {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: props.value,
            onchange: props.onchange,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Die => {
                self.onchange.emit(CallbackMsg::Die(self.value.clone()));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.value != props.value;
        self.value = props.value;
        changed
    }

    fn view(&self) -> Html {
        let (x, y) = (
            self.value.line * (universe::CELL_SIZE as i64),
            self.value.column * (universe::CELL_SIZE as i64),
        );
        let style = format!("top: {}px; left: {}px", x, y);
        let onclick = self.link.callback(|_| Msg::Die);
        html! {
            <div onclick={onclick} class=classes!("app-entity", "red", "darken-1") style={ style }>
            </div>
        }
    }
}
