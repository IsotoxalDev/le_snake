use leptos::{ev::Event, leptos_dom::helpers::IntervalHandle, *};
use std::time::Duration;
use wasm_bindgen::JsValue;

const DURATION: Duration = Duration::from_millis(450);
const SIZE: (usize, usize) = (16, 16);

mod board;
mod game;
mod input;
mod menu;
mod types;

use board::*;
use game::*;
use input::*;
use menu::*;
use types::*;

#[component]
fn Snake(cx: Scope) -> impl IntoView {
    let (state, set_state) = create_signal(cx, [[CellState::Empty; SIZE.0]; SIZE.1]);
    let (game_state, set_game_state) = create_signal(cx, GameState::Playing);
    let (score, set_score) = create_signal(cx, 0);
    let (head, set_head) = create_signal(cx, (2, 0));
    let (towards, set_towards) = create_signal(cx, vec![Direction::Right]);

    // GameLoop
    let game_loop = move || {
        handle_motion(
            set_state,
            game_state,
            set_game_state,
            set_score,
            head,
            set_head,
            towards,
            set_towards,
        )
    };
    let interval = move || {
        return set_interval_with_handle(game_loop, DURATION).unwrap();
    };
    let mut handle = create_rw_signal(cx, interval());

    let mut initial_state = move || {
        // Clear Board;
        set_state([[CellState::Empty; SIZE.0]; SIZE.1]);

        // Set Score to 0
        set_score(0);

        // Initial Snake
        set_state.update(|s: &mut CellData| {
            s[0][0] = CellState::Snake(SnakePart::Tail, Direction::Left)
        });
        set_state.update(|s: &mut CellData| {
            s[0][1] = CellState::Snake(SnakePart::Body, Direction::Left)
        });
        set_state.update(|s: &mut CellData| {
            s[0][2] = CellState::Snake(SnakePart::Head, Direction::Left)
        });

        // Set Head
        set_head((2, 0));

        // Set initial towards direction
        set_towards(vec![Direction::Right]);

        // Initial Apple
        set_state.update(|s| {
            spawn_apple(s);
        });
    };
    initial_state();

    // KeyEvent
    window_event_listener("keydown", move |ev| {
        handle_input(ev, towards, set_towards, game_state, set_game_state)
    });

    // State Management

    create_effect(cx, move |prev| {
        if let Some(p) = prev {
            if game_state() == p {
                return p;
            }
            match game_state() {
                GameState::GameOver | GameState::Paused => handle.update(|h| h.clear()),
                GameState::Playing => match p {
                    GameState::GameOver | GameState::Paused => handle.set(interval()),
                    GameState::Playing => unreachable!(),
                },
            }
        }
        game_state()
    });

    view! { cx,
        <div id="Snake">
            <h1>{score}</h1>
            <Board board_state=state/>
            <GameOver set_game_state game_state score initial_state/>
            <GamePaused set_game_state game_state score initial_state/>
        </div>
    }
}

fn main() {
    mount_to_body(move |cx| view! { cx, <Snake/> })
}
