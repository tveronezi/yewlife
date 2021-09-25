use yew::{classes, html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    Click,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub icon: String,
    pub pulse: bool,
    pub onclick: Callback<()>,
}

pub struct ActionButton {
    link: ComponentLink<Self>,
    icon: String,
    pub pulse: bool,
    onclick: Callback<()>,
}

impl Component for ActionButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            icon: props.icon,
            pulse: props.pulse,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.onclick.emit(());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.icon != props.icon;
        self.pulse = props.pulse;
        self.icon = props.icon;
        changed
    }

    fn view(&self) -> Html {
        let icon = self.icon.clone();
        let (pulse, waves_effect) = if self.pulse {
            (Some("pulse"), None)
        } else {
            (None, Some("waves-effect"))
        };
        let on_click = self.link.callback(|_| Msg::Click);
        html! {
            <a  onclick={on_click}
                class=classes!("btn-floating", "btn-large", waves_effect, "waves-light", "red", pulse)
            >
                <i class="material-icons">{ icon }</i>
            </a>
        }
    }
}
