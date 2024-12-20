use crate::models::Direction;
use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::UseStateHandle;

/// Maps keyboard keys to game directions
pub fn get_direction_from_key(key: &str) -> Option<Direction> {
    match key {
        "ArrowUp" => Some(Direction::Up),
        "ArrowDown" => Some(Direction::Down),
        "ArrowLeft" => Some(Direction::Left),
        "ArrowRight" => Some(Direction::Right),
        _ => None,
    }
}

/// Sets up keyboard event listeners for game controls
pub fn setup_keyboard_controls(current_direction: UseStateHandle<Direction>) -> EventListener {
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();

    let handler = move |event: &web_sys::Event| {
        let event = event.dyn_ref::<KeyboardEvent>().unwrap();
        if let Some(new_direction) = get_direction_from_key(&event.key()) {
            current_direction.set(new_direction);
        }
    };

    EventListener::new(&document, "keydown", handler)
}