use js_sys::Reflect::get;
use leptos::{ev::Event, *};
use wasm_bindgen::JsValue;

use super::{Direction, GameState};

pub fn handle_input(
    ev: Event,
    towards: ReadSignal<Vec<Direction>>,
    set_towards: WriteSignal<Vec<Direction>>,
    game_state: ReadSignal<GameState>,
    set_game_state: WriteSignal<GameState>,
) {
    match game_state() {
        GameState::Paused | GameState::GameOver => return,
        GameState::Playing => {}
    }
    let value = JsValue::from_str("key");
    let key = get(&ev, &value).unwrap().as_string().unwrap();
    let value = JsValue::from_str("repeat");
    if get(&ev, &value).unwrap().as_bool().unwrap() {
        return;
    }
    let dir = match key.as_str() {
        "ArrowDown" => Some(Direction::Down),
        "ArrowUp" => Some(Direction::Up),
        "ArrowRight" => Some(Direction::Right),
        "ArrowLeft" => Some(Direction::Left),
        "Escape" => {
            set_game_state(GameState::Paused);
            None
        }
        _ => None,
    };
    if let Some(d) = dir {
        let last = towards();
        let last = last.last().unwrap();
        if last.clone() != d.opposite() && last.clone() != d {
            set_towards.update(|t| t.push(d))
        }
    }
}
