use yew::{
    classes, html, Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender,
};

#[derive(Debug)]
pub enum Msg {
    Click(MouseEvent),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub icon: String,
    pub pulse: bool,
    pub size: Size,
    pub onclick: Callback<MouseEvent>,
    pub class: Option<String>,
}

#[derive(Clone)]
pub enum Size {
    Large,
    Small,
}

pub struct ActionButton {
    link: ComponentLink<Self>,
    icon: String,
    pulse: bool,
    size: Size,
    pub class: Option<String>,
    onclick: Callback<MouseEvent>,
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
            size: props.size,
            class: props.class,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(event) => {
                self.onclick.emit(event);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.icon != props.icon || self.class != props.class;
        self.pulse = props.pulse;
        self.icon = props.icon;
        self.class = props.class;
        changed
    }

    fn view(&self) -> Html {
        log::info!("button");
        let icon = self.icon.clone();
        let (pulse, waves_effect) = if self.pulse {
            (Some("pulse"), None)
        } else {
            (None, Some("waves-effect"))
        };
        let on_click = self.link.callback(Msg::Click);
        let (btn_size, btn_color, btn_accent) = match self.size {
            Size::Large => ("btn-large", "red", None),
            Size::Small => ("btn-small", "red", Some("accent-1")),
        };
        let cls = &self.class;
        html! {
            <a  onclick={on_click}
                class=classes!("btn-floating", btn_size, waves_effect, "waves-light", btn_color, btn_accent, pulse, cls)
            >
                <i class="material-icons">{ icon }</i>
            </a>
        }
    }
}
