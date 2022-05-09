use crate::components::icons::{Delete, Gear, Play, Refresh};
use yew::{html, Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    PauseOrPlay,
    Random,
    Clear,
    Settings,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub onclick: Callback<Msg>,
    pub state: State,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SettingsState {
    Open,
    Closed,
}

#[derive(Clone, Eq, PartialEq)]
pub enum State {
    Playing,
    Paused,
}

pub struct Actions {
    link: ComponentLink<Self>,
    onclick: Callback<Msg>,
    state: State,
    settings_state: SettingsState,
    previous_settings_state: SettingsState,
}

impl Component for Actions {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            onclick: props.onclick,
            state: props.state,
            settings_state: SettingsState::Closed,
            previous_settings_state: SettingsState::Closed,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PauseOrPlay => {
                self.state = match self.state {
                    State::Playing => State::Paused,
                    State::Paused => State::Playing,
                };
            }
            Msg::Random => {
                self.state = State::Paused;
            }
            Msg::Clear => {
                self.state = State::Paused;
            }
            Msg::Settings => {
                self.previous_settings_state = self.settings_state.clone();
                self.settings_state = match self.settings_state {
                    SettingsState::Open => SettingsState::Closed,
                    SettingsState::Closed => SettingsState::Open,
                };
            }
        };
        self.onclick.emit(msg);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.state != props.state || self.settings_state != self.previous_settings_state
    }

    fn view(&self) -> Html {
        let on_clear_click = self.link.callback(|_| Msg::Clear);
        let on_play_pause_click = self.link.callback(|_| Msg::PauseOrPlay);
        let on_shuffle_click = self.link.callback(|_| Msg::Random);
        let on_settings_click = self.link.callback(|e: MouseEvent| {
            e.cancel_bubble();
            Msg::Settings
        });
        let (_icon, _pulse) = match self.state {
            State::Paused => ("play_arrow", true),
            State::Playing => ("pause", false),
        };
        let _children_hidden = match self.settings_state {
            SettingsState::Open => "".to_string(),
            SettingsState::Closed => " -translate-y-0".to_string(),
        };
        html! {
            <div class="fixed bottom-4 right-4">
                <div class="flex items-end gap-2">
                    <div class="flex flex-col items-center gap-2">
                        <button onclick={ on_clear_click } class="bg-yellow-600 rounded-full p-2">
                            <Delete svg_class={"h-6 w-6 stroke-white".to_string()} path_class={"fill-transparent".to_string()} />
                        </button>
                        <button onclick={ on_shuffle_click } class="bg-yellow-600 rounded-full p-2">
                            <Refresh svg_class={"h-6 w-6 stroke-white".to_string()} path_class={"fill-transparent".to_string()} />
                        </button>
                        <button onclick={ on_settings_click } class="bg-yellow-600 rounded-full">
                            <Gear svg_class={"h-14 w-14 stroke-white".to_string()} path_class={"fill-transparent".to_string()} />
                        </button>
                    </div>
                    <button onclick={ on_play_pause_click } class="h-14 w-14 grid place-content-center bg-red-800 rounded-full">
                        <Play svg_class={"h-14 w-14 stroke-white stroke-1".to_string()} path_class={"fill-transparent".to_string()} />
                    </button>
                </div>
            </div>
        }
    }
}
