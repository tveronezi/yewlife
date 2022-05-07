use yew::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    // empty
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub svg_class: Option<String>,
    pub path_class: Option<String>,
}

pub struct Play {
    pub props: Props,
}

impl Component for Play {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _message: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class=classes!(self.props.svg_class.clone())
                viewBox="0 0 24 24"
                aria-hidden="true"
                data-testid="icon"
                >
                <path class=classes!(self.props.path_class.clone()) stroke-linecap="round" stroke-linejoin="round" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                <path class=classes!(self.props.path_class.clone()) stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
        }
    }
}

pub struct Gear {
    pub props: Props,
}

impl Component for Gear {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _message: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class=classes!(self.props.svg_class.clone())
                fill="none" viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="1"
            >
                <path class=classes!(self.props.path_class.clone()) stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path class=classes!(self.props.path_class.clone()) stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
        }
    }
}

pub struct Delete {
    pub props: Props,
}

impl Component for Delete {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _message: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class=classes!(self.props.svg_class.clone())
                viewBox="0 0 24 24"
                aria-hidden="true"
                data-testid="icon"
            >
                <path
                    class=classes!(self.props.path_class.clone())
                     d="M6 18L18 6M6 6l12 12"
                />
            </svg>
        }
    }
}

pub struct Refresh {
    pub props: Props,
}

impl Component for Refresh {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _message: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class=classes!(self.props.svg_class.clone())
                viewBox="0 0 24 24"
                aria-hidden="true"
                data-testid="icon"
            >
                <path
                    class=classes!(self.props.path_class.clone())
                    d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                />
            </svg>
        }
    }
}
