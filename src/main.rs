mod components;
mod universe;

fn main() {
    // wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<components::app::App>();
}
