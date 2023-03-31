use super::{CellData, CellState, Direction, GameState, SnakePart, SIZE};
use leptos::{leptos_dom::helpers::IntervalHandle, *};
use rand::Rng;

pub fn handle_motion(
    set_state: WriteSignal<CellData>,
    game_state: ReadSignal<GameState>,
    set_game_state: WriteSignal<GameState>,
    set_score: WriteSignal<usize>,
    head: ReadSignal<(usize, usize)>,
    set_head: WriteSignal<(usize, usize)>,
    direction: ReadSignal<Vec<Direction>>,
    set_direction: WriteSignal<Vec<Direction>>,
) {
    let mut input_buffer = direction();
    let mut towards = input_buffer.first().unwrap();
    if input_buffer.len() > 1 {
        set_direction.update(|d| {
            d.remove(0);
        });
        input_buffer = direction();
        towards = input_buffer.first().unwrap();
    }
    let mut curr = head();
    let mut prev: Option<(usize, usize)> = None;
    let mut eaten = false;
    set_state.update(|s| loop {
        if let CellState::Snake(part, dir) = s[curr.1][curr.0] {
            match part {
                SnakePart::Body => {}
                SnakePart::Head => {
                    let (x, y) = towards.next_block(curr.0, curr.1);
                    match s[y][x] {
                        CellState::Snake(_, _) => {
                            set_game_state(GameState::GameOver);
                            break;
                        }
                        CellState::Apple => {
                            set_score.update(|x| *x += 1);
                            spawn_apple(s);
                            eaten = true;
                        }
                        CellState::Empty => {}
                    }
                    set_head((x, y));
                    s[y][x] = CellState::Snake(SnakePart::Head, towards.opposite());
                    s[curr.1][curr.0] = CellState::Snake(SnakePart::Body, dir);
                }
                SnakePart::Tail => {
                    if !eaten {
                        let (x, y) = prev.unwrap();
                        s[y][x] = CellState::Snake(SnakePart::Tail, dir);
                        s[curr.1][curr.0] = CellState::Empty;
                    }
                    break;
                }
            }
            prev = Some(curr);
            curr = dir.next_block(curr.0, curr.1);
        } else {
            break;
        }
    });
}

pub fn spawn_apple(state: &mut CellData) {
    let mut rng = rand::thread_rng();
    loop {
        let (x, y) = (rng.gen_range(0..SIZE.0), rng.gen_range(0..SIZE.1));
        if let CellState::Empty = state[y][x] {
            state[y][x] = CellState::Apple;
            break;
        }
    }
}
