mod components;
mod universe;

fn main() {
    yew::Renderer::<components::app::App>::new().render();
}
