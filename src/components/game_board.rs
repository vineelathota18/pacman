use crate::components::cell::Cell;
use crate::models::{Ghost, Position};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameBoardProps {
    pub score: i32,
    pub game_over: bool,
    pub maze: Vec<Vec<u8>>,
    pub pacman_pos: Position,
    pub ghosts: Vec<Ghost>,
    pub is_dying: bool,
    pub is_invincible: bool,
}

#[function_component]
pub fn GameBoard(props: &GameBoardProps) -> Html {
    let style = format!(
        "grid-template-columns: repeat({}, 1fr);",
        props.maze[0].len()
    );

    html! {
        <>
        <style>
                {include_str!("../styles/game.css")}
            </style>
        <div class="maze" {style}>
            {
                props.maze.iter().enumerate().map(|(y, row)| {
                    row.iter().enumerate().map(|(x, &cell)| {
                        let is_pacman = x == props.pacman_pos.x && y == props.pacman_pos.y;
                        let ghost = props.ghosts.iter()
                            .find(|g| g.position.x == x && g.position.y == y)
                            .cloned();

                        html! {
                            <Cell
                                cell_type={cell}
                                {is_pacman}
                                {ghost}
                                is_dying={props.is_dying && is_pacman}
                                is_invincible={props.is_invincible}
                            />
                        }
                    }).collect::<Html>()
                }).collect::<Html>()
            }
        </div>
        </>
    }
}
