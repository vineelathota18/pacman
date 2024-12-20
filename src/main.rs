mod app;
mod components;
mod constants;
mod game_logic;
mod models;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
