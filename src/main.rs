use yew::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq)]
struct Ghost {
    position: Position,
    color: &'static str,
}

#[derive(Properties, PartialEq)]
struct CellProps {
    cell_type: u8,
    is_pacman: bool,
    ghost: Option<Ghost>,
}

#[function_component]
fn Cell(props: &CellProps) -> Html {
    let class = classes!(
        "cell",
        if props.is_pacman {
            "pacman"
        } else if props.ghost.is_some() {
            "ghost"
        } else {
            match props.cell_type {
                1 => "wall",
                2 => "dot",
                3 => "power-pellet",
                _ => "empty",
            }
        }
    );

    let content = if props.is_pacman {
        html! {
            <div class="pacman-body">
                <div class="pacman-eye"></div>
            </div>
        }
    } else if let Some(ghost) = &props.ghost {
        let style = format!("background-color: {};", ghost.color);
        html! {
            <div class="ghost-body" {style}>{"👻"}</div>
        }
    } else {
        match props.cell_type {
            2 => html! { "." },
            3 => html! { "o" },
            _ => html! { "" },
        }
    };

    html! {
        <div class={class}>
            {content}
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let maze = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1],
        vec![1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1],
        vec![1, 3, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 3, 1],
        vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
        vec![1, 2, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 2, 1],
        vec![1, 2, 2, 2, 2, 1, 2, 2, 2, 1, 1, 2, 2, 2, 1, 2, 2, 2, 2, 1],
        vec![1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 1, 1, 1],
        vec![1, 2, 2, 2, 2, 2, 2, 1, 1, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 1],
        vec![1, 2, 2, 2, 2, 2, 2, 1, 1, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 1],
        vec![1, 1, 1, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1],
        vec![1, 2, 2, 2, 2, 1, 2, 2, 2, 1, 1, 2, 2, 2, 1, 2, 2, 2, 2, 1],
        vec![1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1],
        vec![1, 3, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 3, 1],
        vec![1, 1, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 1, 1],
        vec![1, 2, 2, 2, 2, 1, 2, 2, 2, 1, 1, 2, 2, 2, 1, 2, 2, 2, 2, 1],
        vec![1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let mut rng = rand::thread_rng();
    let mut valid_positions: Vec<Position> = maze.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell != 1 {
                    Some(Position { x, y })
                } else {
                    None
                }
            })
        })
        .collect();

    // First select Pacman's position
    let pacman_idx = rng.gen_range(0..valid_positions.len());
    let pacman_pos = valid_positions.remove(pacman_idx);

    // Then place ghosts in remaining positions
    let ghost_colors = vec!["#FF0000", "#00FFFF", "#FFB8FF", "#FFB852"];
    let ghosts = ghost_colors.iter()
        .filter_map(|&color| {
            if valid_positions.is_empty() {
                None
            } else {
                let idx = rng.gen_range(0..valid_positions.len());
                let position = valid_positions.remove(idx);
                Some(Ghost { position, color })
            }
        })
        .collect::<Vec<_>>();

    let style = format!("grid-template-columns: repeat({}, 1fr);", maze[0].len());

    html! {
        <>
            <style>
                {r#"
                    .maze {
                        display: grid;
                        margin: 20px auto;
                        border: 2px solid #333;
                        background-color: #000;
                        padding: 10px;
                    }
                    .cell {
                        width: 100%;
                        aspect-ratio: 1;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-family: 'Arial', sans-serif;
                        font-weight: bold;
                        font-size: 20px;
                    }
                    .wall {
                        background-color: #00f;
                        border: 1px solid #000;
                    }
                    .empty {
                        background-color: #000;
                    }
                    .dot {
                        background-color: #000;
                        color: #fff;
                    }
                    .power-pellet {
                        background-color: #000;
                        color: #fff;
                        font-size: 24px;
                    }
                    .pacman {
                        background-color: #ffff00;
                        border-radius: 90%;
                    }
                    .pacman-body {
                        background: #030303;
                        width: 80%;
                        height: 80%;
                        border-radius: 50%;
                        position: relative;
                        clip-path: polygon(100% 0, 100% 100%, 50% 50%, 100% 0);
                        animation: eat 0.4s linear infinite;
                    }
                    .pacman-eye {
                        position: absolute;
                        width: 4px;
                        height: 4px;
                        border-radius: 50%;
                        background: #000;
                        top: 25%;
                        right: 25%;
                    }
                    @keyframes eat {
                        0% { clip-path: polygon(100% 15%, 100% 85%, 50% 50%, 100% 15%); }
                        50% { clip-path: polygon(100% 50%, 100% 50%, 50% 50%, 100% 50%); }
                        100% { clip-path: polygon(100% 15%, 100% 85%, 50% 50%, 100% 15%); }
                    }
                    .ghost {
                        background-color: #000;
                    }
                    .ghost-body {
                        width: 80%;
                        height: 80%;
                        border-radius: 50% 50% 0 0;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        animation: float 1s ease-in-out infinite;
                    }
                    @keyframes chomp {
                        0% { transform: rotate(0deg); }
                        50% { transform: rotate(45deg); }
                        100% { transform: rotate(0deg); }
                    }
                    @keyframes float {
                        0%, 100% { transform: translateY(0); }
                        50% { transform: translateY(-3px); }
                    }
                "#}
            </style>
            <div class="maze" {style}>
                {
                    maze.iter().enumerate().map(|(y, row)| {
                        row.iter().enumerate().map(|(x, &cell)| {
                            let is_pacman = x == pacman_pos.x && y == pacman_pos.y;
                            let ghost = ghosts.iter()
                                .find(|g| g.position.x == x && g.position.y == y)
                                .cloned();
                            
                            html! {
                                <Cell 
                                    cell_type={cell} 
                                    is_pacman={is_pacman}
                                    ghost={ghost}
                                />
                            }
                        }).collect::<Html>()
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}