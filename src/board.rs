use leptos::*;

use super::{CellData, CellState};

#[component]
pub fn Cell<S>(cx: Scope, state: S) -> impl IntoView
where
    S: Fn() -> CellState + 'static,
{
    view! { cx,
        <div class=move || {
            format!(
                "cell {}", match state() { CellState::Empty => "", CellState::Apple => "apple",
                CellState::Snake(_, _) => "snake", }
            )
        }></div>
    }
}

#[component]
pub fn Board(cx: Scope, board_state: ReadSignal<CellData>) -> impl IntoView {
    let get_state = move |x: usize, y: usize| board_state()[x][y];
    view! { cx,
        {(0..16)
            .into_iter()
            .map(|x| {
                view! { cx,
                    <div class="row">
                        {(0..16)
                            .into_iter()
                            .map(|y| {
                                view! { cx, <Cell state=move || get_state(x, y)/> }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                }
            })
            .collect::<Vec<_>>()}
    }
}
