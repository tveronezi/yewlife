use crate::components::bean::Bean;
use crate::universe;
use yew::{classes, html, utils, Component, ComponentLink, Html, MouseEvent, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    Tick,
    AddEntity(MouseEvent),
}

pub struct Existence {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: universe::Universe,
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
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let window: web_sys::Window = utils::window();
        let window_height = window
            .inner_height()
            .expect("Unable to load window height")
            .as_f64()
            .expect("height is not a number")
            .round() as i64;
        let window_width = window
            .inner_width()
            .expect("Unable to load window width")
            .as_f64()
            .expect("width is not a number")
            .round() as i64;
        let entities = self
            .value
            .entities
            .iter()
            .filter(|e| {
                if e.column < 0 || e.line < 0 {
                    return false;
                }
                if e.column * (universe::CELL_SIZE as i64) > window_width
                    || e.line * (universe::CELL_SIZE as i64) > window_height
                {
                    return false;
                }
                true
            })
            .map(|e| {
                html! {
                    <Bean value={e.clone()} />
                }
            })
            .collect::<Html>();
        let on_tick_click = self.link.callback(|_| Msg::Tick);
        let on_existence_click = self.link.callback(Msg::AddEntity);
        html! {
            <div onmousedown={on_existence_click} class=classes!("app-existence")>
                { entities }
                <div class="app-tick">
                    <a  onclick={on_tick_click}
                        class="btn-floating btn-large waves-effect waves-light red"
                    >
                        <i class="material-icons">{ "add" }</i>
                    </a>
                </div>
            </div>
        }
    }
}
