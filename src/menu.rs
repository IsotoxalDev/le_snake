use super::GameState;
use leptos::*;

#[component]
pub fn GameOver<I>(
    cx: Scope,
    set_game_state: WriteSignal<GameState>,
    game_state: ReadSignal<GameState>,
    score: ReadSignal<usize>,
    initial_state: I,
) -> impl IntoView
where
    I: FnOnce() + 'static + Copy + Clone,
{
    let play = move |_| {
        set_game_state(GameState::Playing);
        initial_state();
    };
    view! { cx,
        <Show
            when=move || { if let GameState::GameOver = game_state() { true } else { false } }
            fallback=move |_| {}
        >
            <div class="overlay">
                <div id="GameOver" class="overlay_child">
                    <h1>"Game Over"</h1>
                    <h2>"Score: " {score}</h2>
                    <button on:click=play>"Play Again!"</button>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn GamePaused<I>(
    cx: Scope,
    set_game_state: WriteSignal<GameState>,
    game_state: ReadSignal<GameState>,
    score: ReadSignal<usize>,
    initial_state: I,
) -> impl IntoView
where
    I: FnOnce() + 'static + Copy + Clone,
{
    let play = move |_| {
        set_game_state(GameState::Playing);
    };
    view! { cx,
        <Show
            when=move || { if let GameState::Paused = game_state() { true } else { false } }
            fallback=move |_| {}
        >
            <div class="overlay">
                <div id="GamePaused" class="overlay_child">
                    <h1>"Game Paused"</h1>
                    <button on:click=play>"Resume"</button>
                </div>
            </div>
        </Show>
    }
}
