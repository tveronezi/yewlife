use crate::components::bean::{Bean, CallbackMsg};
use crate::components::button::ActionButton;
use crate::components::score::Score;
use crate::universe;
use gloo::timers::callback::Interval;
use js_sys::Math::random;
use yew::{classes, html, utils, Component, ComponentLink, Html, MouseEvent, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    Tick,
    TickClick,
    Shuffle,
    Clear,
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

fn add_entity(universe: &mut universe::Universe, x: i32, y: i32) {
    let column = x / (universe::CELL_SIZE as i32);
    let line = y / (universe::CELL_SIZE as i32);
    let entity = universe::Entity {
        line: line as i64,
        column: column as i64,
    };
    universe.entities.insert(entity);
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

fn random_universe(universe: &mut universe::Universe) {
    universe.entities.clear();
    let window = window_dimensions();
    for y in 0..(window.height / (universe::CELL_SIZE as i64)) {
        for x in 0..(window.width / (universe::CELL_SIZE as i64)) {
            let number = random().round() as i64;
            if number % 7 == 0 {
                add_entity(
                    universe,
                    (x * (universe::CELL_SIZE as i64)) as i32,
                    (y * (universe::CELL_SIZE as i64)) as i32,
                )
            }
        }
    }
    universe.tick();
}

impl Component for Existence {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut universe = universe::Universe::new(include_str!("init_pattern.txt"));
        universe.tick();
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
                add_entity(&mut self.value, x, y);
                true
            }
            Msg::TickClick => {
                self.timer = match &self.timer {
                    None => {
                        let this_link = self.link.clone();
                        Some(Interval::new(1, move || this_link.send_message(Msg::Tick)))
                    }
                    Some(_) => None,
                };
                true
            }
            Msg::EntityCallback(msg) => match msg {
                CallbackMsg::Die(entity) => self.value.entities.remove(&entity), // if true, it will rerender
            },
            Msg::Shuffle => {
                self.timer = None;
                random_universe(&mut self.value);
                true
            }
            Msg::Clear => {
                self.value.entities.clear();
                true
            }
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
        let on_tick_click = self.link.callback(|_| Msg::TickClick);
        let on_shuffle_click = self.link.callback(|_| Msg::Shuffle);
        let on_clear_click = self.link.callback(|_| Msg::Clear);
        let on_existence_click = self.link.callback(Msg::AddEntity);
        let (icon, pulse) = match self.timer {
            None => ("play_arrow", true),
            Some(_) => ("pause", false),
        };
        let universe = self.value.clone(); // yeah... lazy.
        html! {
            <div onmousedown={on_existence_click} class=classes!("app-existence", "grey", "darken-4")>
                { entities }
                <div class="app-buttons">
                    <div class="app-clear">
                        <ActionButton icon={"clear"} pulse={false} onclick={on_clear_click} />
                    </div>
                    <div class="app-random">
                        <ActionButton icon={"shuffle"} pulse={false} onclick={on_shuffle_click} />
                    </div>
                    <div class="app-tick">
                        <ActionButton icon={icon} pulse={pulse} onclick={on_tick_click} />
                    </div>
                </div>
                <Score universe={ universe }/>
            </div>
        }
    }
}
