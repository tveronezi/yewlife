use yew::{classes, html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    // empty
}

#[derive(Clone, Properties)]
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
                <path
                    class=classes!(self.props.path_class.clone())
                    d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"
                />
            </svg>
        }
    }
}

/*
<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
  <path stroke-linecap="round" stroke-linejoin="round" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
  <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
</svg>
 */

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
                viewBox="0 0 24 24"
                aria-hidden="true"
                data-testid="icon"
            >
                <path
                    class=classes!(self.props.path_class.clone())
                     d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z"
                />
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
