use rand::Rng;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use web_sys::KeyboardEvent;
use gloo::events::EventListener;
use wasm_bindgen::JsCast;

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
            <>
            <div class="pacman-body"></div>
            <div class="pacman-eye"></div>
            </>
        }
    } else if let Some(ghost) = &props.ghost {
        let style = format!("background-color: {};", ghost.color);
        html! {
            <div class="ghost-body" {style}>{"👻"}</div>
        }
    } else {
        match props.cell_type {
            2 => html! { "." },
            3 => html! { "⚪" },
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
    let maze = use_state(|| vec![
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
    ]);

    let pacman_pos = use_state(|| Position { x: 1, y: 1 });
    
    let ghosts = use_state(|| {
        let mut rng = rand::thread_rng();
        let ghost_colors = vec!["#FF0000", "#00FFFF", "#FFB8FF", "#FFB852"];
        let mut valid_positions: Vec<Position> = maze.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, &cell)| {
                    if cell != 1 && !(x == 1 && y == 1) {
                        Some(Position { x, y })
                    } else {
                        None
                    }
                })
            })
            .collect();

        ghost_colors.iter()
            .filter_map(|&color| {
                if valid_positions.is_empty() {
                    None
                } else {
                    let idx = rng.gen_range(0..valid_positions.len());
                    let position = valid_positions.remove(idx);
                    Some(Ghost { position, color })
                }
            })
            .collect::<Vec<_>>()
    });

    // Add keyboard event listener
    {
        let pacman_pos = pacman_pos.clone();
        let maze = maze.clone();

        use_effect(move || {
            let document = web_sys::window()
                .unwrap()
                .document()
                .unwrap();

            let handler = move |event: &web_sys::Event| {
                let event = event.dyn_ref::<KeyboardEvent>().unwrap();
                let mut new_pos = (*pacman_pos).clone();

                match event.key().as_str() {
                    "ArrowUp" => {
                        if new_pos.y > 0 && maze[new_pos.y - 1][new_pos.x] != 1 {
                            new_pos.y -= 1;
                        }
                    }
                    "ArrowDown" => {
                        if new_pos.y < maze.len() - 1 && maze[new_pos.y + 1][new_pos.x] != 1 {
                            new_pos.y += 1;
                        }
                    }
                    "ArrowLeft" => {
                        if new_pos.x > 0 && maze[new_pos.y][new_pos.x - 1] != 1 {
                            new_pos.x -= 1;
                        }
                    }
                    "ArrowRight" => {
                        if new_pos.x < maze[0].len() - 1 && maze[new_pos.y][new_pos.x + 1] != 1 {
                            new_pos.x += 1;
                        }
                    }
                    _ => return,
                }

                pacman_pos.set(new_pos);
            };

            let listener = EventListener::new(&document, "keydown", handler);
            
            || drop(listener)
        });
    }

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
                        outline: 1px solid #000;
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
                        position: relative;
                    }
                    .pacman-body {
                        background: #030303;
                        width: 100%;
                        height: 100%;
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
                        top: 20%;
                        right: 32%;
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
                            let ghost = (*ghosts).iter()
                                .find(|g| g.position.x == x && g.position.y == y)
                                .cloned();

                            html! {
                                <Cell
                                    cell_type={cell}
                                    {is_pacman}
                                    {ghost}
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