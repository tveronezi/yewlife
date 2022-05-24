use super::Dimensions;
use crate::universe::{Entity, Universe, CELL_SIZE};
use rand::Rng;
use std::rc::Rc;
use yew::prelude::*;

fn random_universe(universe: &mut Universe) {
    universe.entities.clear();
    let window = window_dimensions();
    let max_y = window.height / CELL_SIZE;
    let max_x = window.width / CELL_SIZE;
    let mut rng = rand::thread_rng();
    for y in 0..max_y {
        for x in 0..max_x {
            let number = rng.gen_range(0..10);
            if number == 7 {
                add_entity(universe, x * CELL_SIZE, y * CELL_SIZE)
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

fn add_entity(universe: &mut Universe, x: i32, y: i32) {
    let column = x / CELL_SIZE;
    let line = y / CELL_SIZE;
    let entity = Entity { line, column };
    universe.entities.insert(entity);
}

pub enum Command {
    Shuffle,
    Clear,
    Play,
}

impl Reducible for Universe {
    type Action = Command;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Rc::new(match action {
            Command::Shuffle => {
                let mut universe = Universe::new("");
                random_universe(&mut universe);
                universe.tick();
                universe
            }
            Command::Clear => Universe::new(""),
            Command::Play => {
                let mut universe = (*self).clone();
                universe.tick();
                universe
            }
        })
    }
}

pub type MessageContext = UseReducerHandle<Universe>;

#[derive(Properties, Debug, PartialEq)]
pub struct UniverseProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(UniverseProvider)]
pub fn provider(props: &UniverseProviderProps) -> Html {
    let universe = use_reducer(|| {
        let mut universe = Universe::new("");
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
