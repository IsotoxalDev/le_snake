use leptos::{ev::Event, leptos_dom::helpers::IntervalHandle, *};
use std::time::Duration;
use wasm_bindgen::JsValue;

const DURATION: Duration = Duration::from_millis(450);
const SIZE: (usize, usize) = (16, 16);

mod board;
mod game;
mod input;
mod menu;

use board::*;
use game::*;
use input::*;
use menu::*;

/// Enum for different parts of snake.
#[derive(Copy, Clone, PartialEq)]
pub enum SnakePart {
    Body,
    Head,
    Tail,
}

/// Enum for different types of states a cell can have
#[derive(Copy, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Apple,
    Snake(SnakePart, Direction),
}

/// Enum for state of game.
#[derive(Copy, Clone, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

/// Type export to easily create an array of CellState
pub type CellData = [[CellState; SIZE.0]; SIZE.1];

/// Enum for direction
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    /// Get opposite direction
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
    /// get the next block based on the give coordinates
    pub fn next_block(&self, x: usize, y: usize) -> (usize, usize) {
        let (mut temp_x, mut temp_y) = match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        };
        temp_x = x as isize + temp_x;
        temp_y = y as isize + temp_y;

        // Loop the snake if out of bounds
        temp_x = if temp_x > SIZE.0 as isize - 1 {
            temp_x - SIZE.0 as isize
        } else if temp_x < 0 {
            temp_x + SIZE.0 as isize
        } else {
            temp_x
        };
        temp_y = if temp_y > SIZE.1 as isize - 1 {
            temp_y - SIZE.1 as isize
        } else if temp_y < 0 {
            temp_y + SIZE.1 as isize
        } else {
            temp_y
        };
        (temp_x as usize, temp_y as usize)
    }
}

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
