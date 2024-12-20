mod app;
mod components;
mod constants;
mod controls;
mod game_logic;
mod models;
mod tests;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
