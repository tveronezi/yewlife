use crate::components::actions::{self, Actions};
use crate::components::bean::{Bean, CallbackMsg};
use crate::components::score::Score;
use crate::universe;
use gloo::timers::callback::Interval;
use js_sys::Math::random;
use yew::{html, utils, web_sys, Component, ComponentLink, Html, MouseEvent, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    Tick,
    Action(actions::Msg),
    ExistenceClick(MouseEvent),
    EntityCallback(crate::components::bean::CallbackMsg),
}

pub struct Existence {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: Box<universe::Universe>,
    timer: Option<Interval>,
    state: State,
}

#[derive(Clone)]
enum State {
    Playing,
    Paused,
}

fn add_entity(universe: &mut universe::Universe, x: i32, y: i32) {
    let column = x / universe::CELL_SIZE;
    let line = y / universe::CELL_SIZE;
    let entity = universe::Entity { line, column };
    universe.entities.insert(entity);
}

pub fn is_visible(e: &universe::Entity) -> bool {
    if e.column < 0 || e.line < 0 {
        return false;
    }
    let window = window_dimensions();
    let outside_width = e.column * universe::CELL_SIZE > window.width;
    let outside_height = e.line * universe::CELL_SIZE > window.height;
    !(outside_height || outside_width)
}

struct Dimensions {
    height: i32,
    width: i32,
}

fn window_dimensions() -> Dimensions {
    let window: web_sys::Window = utils::window();
    let height = window
        .inner_height()
        .expect("Unable to load window height")
        .as_f64()
        .expect("height is not a number")
        .round() as i32;
    let width = window
        .inner_width()
        .expect("Unable to load window width")
        .as_f64()
        .expect("width is not a number")
        .round() as i32;
    Dimensions { height, width }
}

fn random_universe(universe: &mut universe::Universe) {
    universe.entities.clear();
    let window = window_dimensions();
    let max_y = window.height / universe::CELL_SIZE;
    let max_x = window.width / universe::CELL_SIZE;
    for y in 0..max_y {
        for x in 0..max_x {
            let number = random().round() as i64;
            if number % 7 == 0 {
                add_entity(universe, x * universe::CELL_SIZE, y * universe::CELL_SIZE)
            }
        }
    }
    universe.tick();
}

impl Component for Existence {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut universe = universe::Universe::new("");
        random_universe(&mut universe);
        universe.tick();
        Self {
            link,
            value: Box::new(universe),
            timer: None,
            state: State::Paused,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                self.value.tick();
                true
            }
            Msg::ExistenceClick(event) => {
                if event.target() != event.current_target() {
                    return false;
                }
                let x = event.x();
                let y = event.y();
                add_entity(&mut self.value, x, y);
                true
            }
            Msg::EntityCallback(msg) => match msg {
                CallbackMsg::Die(entity) => self.value.entities.remove(&entity), // if true, it will rerender
            },
            Msg::Action(message) => match message {
                actions::Msg::PauseOrPlay => match self.state {
                    State::Playing => {
                        self.timer = None;
                        self.state = State::Paused;
                        true
                    }
                    State::Paused => {
                        let link = self.link.clone();
                        self.timer = Some(Interval::new(0, move || link.send_message(Msg::Tick)));
                        self.state = State::Playing;
                        true
                    }
                },
                actions::Msg::Random => {
                    self.timer = None;
                    self.state = State::Paused;
                    random_universe(&mut self.value);
                    true
                }
                actions::Msg::Clear => {
                    self.timer = None;
                    self.state = State::Paused;
                    self.value.entities.clear();
                    true
                }
                actions::Msg::Settings => true,
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        log::info!("existence");
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
        let on_action_click = self.link.callback(Msg::Action);
        let on_existence_click = self.link.callback(Msg::ExistenceClick);
        let universe = self.value.clone();
        let state = match self.state {
            State::Playing => actions::State::Playing,
            State::Paused => actions::State::Paused,
        };
        html! {
            <div onclick={on_existence_click} class="app-existence bg-black">
                { entities }
                <Actions onclick={ on_action_click } state={ state }/>
                <Score universe={ universe }/>
            </div>
        }
    }
}
