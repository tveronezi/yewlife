use crate::universe;
use crate::universe::CELL_SIZE;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

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
        let (x, y) = (self.value.line * CELL_SIZE, self.value.column * CELL_SIZE);
        let style = format!(
            "top: {}px; left: {}px; height: {}px; width: {}px",
            x, y, CELL_SIZE, CELL_SIZE
        );
        let onclick = self.link.callback(|_| Msg::Die);
        html! {
            <div onclick={onclick} class="absolute bg-red-400 bg-red-700" style={ style }/>
        }
    }
}

/*
.app-entity
  position: absolute
  height: 10px
  width: 10px
  border: solid 1px #212121
  > div
    width: 100%
    height: 100%

 */
