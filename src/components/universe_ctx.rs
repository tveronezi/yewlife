use rand::Rng;
use std::rc::Rc;

use yew::prelude::*;

use crate::universe;

use super::Dimensions;

fn random_universe(universe: &mut universe::Universe) {
    universe.entities.clear();
    let window = window_dimensions();
    let max_y = window.height / universe::CELL_SIZE;
    let max_x = window.width / universe::CELL_SIZE;
    let mut rng = rand::thread_rng();
    for y in 0..max_y {
        for x in 0..max_x {
            let number = rng.gen_range(0..10);
            if number == 7 {
                add_entity(universe, x * universe::CELL_SIZE, y * universe::CELL_SIZE)
            }
        }
    }
    universe.tick();
}

pub fn window_dimensions() -> Dimensions {
    let window = gloo_utils::window();
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

fn add_entity(universe: &mut universe::Universe, x: i32, y: i32) {
    let column = x / universe::CELL_SIZE;
    let line = y / universe::CELL_SIZE;
    let entity = universe::Entity { line, column };
    universe.entities.insert(entity);
}

pub enum Command {
    Shuffle,
    Clear,
    Play,
    Tick,
    Iddle,
}

impl Reducible for universe::Universe {
    type Action = Command;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Rc::new(match action {
            Command::Shuffle => {
                let mut universe = universe::Universe::new("");
                random_universe(&mut universe);
                universe.tick();
                universe
            }
            Command::Clear => universe::Universe::new(""),
            Command::Play => {
                let mut universe = (*self).clone();
                universe.tick();
                universe
            }
            Command::Tick => {
                let mut universe = (*self).clone();
                universe.tick();
                universe
            }
            Command::Iddle => (*self).clone(),
        })
    }
}

pub type MessageContext = UseReducerHandle<universe::Universe>;

#[derive(Properties, Debug, PartialEq)]
pub struct UniverseProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(UniverseProvider)]
pub fn provider(props: &UniverseProviderProps) -> Html {
    let universe = use_reducer(|| {
        let mut universe = universe::Universe::new("");
        random_universe(&mut universe);
        universe.tick();
        universe
    });

    html! {
        <ContextProvider<MessageContext> context={universe}>
            {props.children.clone()}
        </ContextProvider<MessageContext>>
    }
}
