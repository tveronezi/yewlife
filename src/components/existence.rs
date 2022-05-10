use crate::universe;
use rand::Rng;
use yew::prelude::*;

use crate::components::bean::Bean;

struct Dimensions {
    height: i32,
    width: i32,
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

fn is_visible(e: &universe::Entity) -> bool {
    if e.column < 0 || e.line < 0 {
        return false;
    }
    let window = window_dimensions();
    let outside_width = e.column * universe::CELL_SIZE > window.width;
    let outside_height = e.line * universe::CELL_SIZE > window.height;
    !(outside_height || outside_width)
}

#[function_component(Existence)]
pub fn existence() -> Html {
    let universe = use_state(|| {
        let mut universe = universe::Universe::new("");
        random_universe(&mut universe);
        universe.tick();
        universe
    });
    let entities = universe
        .entities
        .iter()
        .filter(|e| is_visible(*e))
        .map(|e| {
            html! {
                <Bean value={e.clone()} />
            }
        })
        .collect::<Html>();
    html! {
        <div>
          {entities}
        </div>
    }
}
