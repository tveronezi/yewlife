use crate::components::bean::{Bean, CallbackMsg};
use crate::components::score::Score;
use crate::universe;
use gloo::timers::callback::Interval;
use yew::{classes, html, utils, Component, ComponentLink, Html, MouseEvent, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    Tick,
    Play,
    Pause,
    AddEntity(MouseEvent),
    EntityCallback(crate::components::bean::CallbackMsg),
}

pub struct Existence {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: universe::Universe,
    timer: Option<Interval>,
}

impl Existence {
    fn add_entity(&mut self, x: i32, y: i32) {
        let column = x / (universe::CELL_SIZE as i32);
        let line = y / (universe::CELL_SIZE as i32);
        let entity = universe::Entity {
            line: line as i64,
            column: column as i64,
        };
        self.value.entities.insert(entity);
    }
}

pub fn is_visible(e: &universe::Entity) -> bool {
    if e.column < 0 || e.line < 0 {
        return false;
    }
    let window = window_dimensions();
    if e.column * (universe::CELL_SIZE as i64) > window.width
        || e.line * (universe::CELL_SIZE as i64) > window.height
    {
        return false;
    }
    true
}

struct Dimensions {
    height: i64,
    width: i64,
}

fn window_dimensions() -> Dimensions {
    let window: web_sys::Window = utils::window();
    let height = window
        .inner_height()
        .expect("Unable to load window height")
        .as_f64()
        .expect("height is not a number")
        .round() as i64;
    let width = window
        .inner_width()
        .expect("Unable to load window width")
        .as_f64()
        .expect("width is not a number")
        .round() as i64;
    Dimensions { height, width }
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
        Self {
            link,
            value: universe,
            timer: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                self.value.tick();
                true
            }
            Msg::AddEntity(event) => {
                let x = event.x();
                let y = event.y();
                self.add_entity(x, y);
                true
            }
            Msg::Play => {
                let this_link = self.link.clone();
                self.timer = Some(Interval::new(1, move || this_link.send_message(Msg::Tick)));
                true
            }
            Msg::Pause => {
                self.timer = None;
                true
            }
            Msg::EntityCallback(msg) => match msg {
                CallbackMsg::Die(entity) => self.value.entities.remove(&entity), // if true, it will rerender
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let entities = self
            .value
            .entities
            .iter()
            .filter(|e| is_visible(*e))
            .map(|e| {
                let onchange = self.link.callback(Msg::EntityCallback);
                html! {
                    <Bean value={e.clone()} onchange={ onchange } />
                }
            })
            .collect::<Html>();
        let running = self.timer.is_some();
        let on_tick_click = self.link.callback(move |_| {
            if running {
                return Msg::Pause;
            }
            Msg::Play
        });
        let on_existence_click = self.link.callback(Msg::AddEntity);
        let (icon, pulse, waves_effect) = match self.timer {
            None => ("play_arrow", Some("pulse"), None),
            Some(_) => ("pause", None, Some("waves-effect")),
        };
        let universe = self.value.clone();
        html! {
            <div onmousedown={on_existence_click} class=classes!("app-existence", "grey", "darken-4")>
                { entities }
                <div class="app-tick">
                    <a  onclick={on_tick_click}
                        class=classes!("btn-floating", "btn-large", waves_effect, "waves-light", "red", pulse)
                    >
                        <i class="material-icons">{ icon }</i>
                    </a>
                </div>
                <Score universe={ universe }/>
            </div>
        }
    }
}
