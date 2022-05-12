use crate::components::actions::Actions;
use crate::components::existence::Existence;
use crate::components::msg_ctx::MessageProvider;
use crate::components::Dimensions;
use crate::universe;
use rand::Rng;
use yew::prelude::*;

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

#[function_component(App)]
pub fn app() -> Html {
    let universe = use_state(|| {
        let mut universe = universe::Universe::new("");
        random_universe(&mut universe);
        universe.tick();
        universe
    });
    let cloned_universe = universe.clone();
    let on_shuffle = Callback::from(move |_| {
        let mut universe = universe::Universe::new("");
        random_universe(&mut universe);
        universe.tick();
        cloned_universe.set(universe);
    });
    let cloned_universe = universe.clone();
    let on_clear = Callback::from(move |_| {
        cloned_universe.set(universe::Universe::new(""));
    });
    let cloned_universe = universe.clone();
    let on_play = Callback::from(move |_| {
        let mut new_universe = (*cloned_universe).clone();
        new_universe.tick();
        cloned_universe.set(new_universe);
    });
    let dimensions = window_dimensions();
    html! {
        <div class="h-screen bg-black">
            <MessageProvider>
                <Existence universe={(*universe).clone()} {dimensions} />
                <Actions {on_clear} {on_play} {on_shuffle} />
            </MessageProvider>
        </div>
    }
}
