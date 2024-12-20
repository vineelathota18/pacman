use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ScoreboardProps {
    pub score: i32,
    pub lives: i32,
    pub restart_timer: bool,
    pub game_over: bool,
    pub on_restart: Callback<()>,
}

#[function_component]
pub fn Scoreboard(props: &ScoreboardProps) -> Html {
    let onclick = {
        let on_restart = props.on_restart.clone();
        Callback::from(move |_: MouseEvent| {  
            on_restart.emit(());
        })
    };

    html! {
        <div class="game-info">
            <div class="score">{"Score: "}{props.score}</div>
            <div class="lives">{"Lives: "}{props.lives}</div>
            {
                if props.restart_timer {
                    html! {
                        <div class="message">{"Get Ready!"}</div>
                    }
                } else if props.game_over {
                    html! {
                        <>
                            <div class="game-over">{"Game Over!"}</div>
                            <button {onclick} class="restart-button">
                                {"Restart Game"}
                            </button>
                        </>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}