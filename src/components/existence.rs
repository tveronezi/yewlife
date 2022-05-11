use crate::universe;
use rand::Rng;
use yew::prelude::*;

use crate::components::bean::Bean;

pub struct Dimensions {
    pub height: i32,
    pub width: i32,
}

fn window_dimensions() -> Dimensions {
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

#[function_component(Existence)]
pub fn existence() -> Html {
    let window = window_dimensions();
    let universe = use_state(|| {
        let mut universe = universe::Universe::new("");
        random_universe(&mut universe);
        universe.tick();
        universe
    });
    let entities = universe
        .entities
        .iter()
        .filter(|e| (*e).is_visible(&window))
        .map(|e| {
            html! {
                <Bean key={format!("c{}-l{}", e.column, e.line)} value={e.clone()} />
            }
        })
        .collect::<Html>();
    html! {
        <div>
          {entities}
        </div>
    }
}
