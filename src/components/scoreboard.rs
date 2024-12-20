use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ScoreboardProps {
    pub score: i32,
    pub lives: i32,
    pub is_invincible: bool,
    pub restart_timer: bool,
    pub game_over: bool,
}

#[function_component]
pub fn Scoreboard(props: &ScoreboardProps) -> Html {
    html! {
        <div class="game-info">
            <div class="score">{"Score: "}{props.score}</div>
            <div class="lives">{"Lives: "}{props.lives}</div>
            <div class="score">{"is_invincible: "}{props.is_invincible}</div>
            {
                if props.restart_timer {
                    html! {
                        <div class="message">{"Get Ready!"}</div>
                    }
                } else if props.game_over {
                    html! {
                        <div class="game-over">{"Game Over!"}</div>
                    }
                } else if props.is_invincible {
                    html! {
                        <div class="power-pellet">{"Power Pellet Active!"}</div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}