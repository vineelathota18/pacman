use rand::Rng;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use web_sys::KeyboardEvent;
use gloo::events::EventListener;
use gloo::timers::callback::{Interval, Timeout};
use wasm_bindgen::JsCast;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
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
    is_dying: bool,
}

#[function_component]
fn Cell(props: &CellProps) -> Html {
    let class = classes!(
        "cell",
        if props.is_pacman {
            if props.is_dying {
                "pacman dying"
            } else {
                "pacman"
            }
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
        vec![1, 0, 2, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1],
        vec![1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1],
        vec![1, 3, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 3, 1],
        vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
        vec![1, 2, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 2, 1],
        vec![1, 2, 2, 2, 2, 1, 2, 2, 2, 1, 1, 2, 2, 2, 1, 2, 2, 2, 2, 1],
        vec![1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 1, 1, 1],
        vec![1, 2, 2, 2, 2, 2, 2, 1, 1, 3, 2, 1, 1, 2, 2, 2, 2, 2, 2, 1],
        vec![1, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 1],
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
    let score = use_state(|| 0);
    let is_dying = use_state(|| false);
    let current_direction = use_state(|| Direction::None);
    let game_over = use_state(|| false);
    
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

    // Add movement interval
    {
        let pacman_pos = pacman_pos.clone();
        let current_direction = current_direction.clone();
        let maze = maze.clone();
        let score = score.clone();
        let game_over = game_over.clone();
        let ghosts = ghosts.clone(); 
        let is_dying_handler = is_dying.clone();

        use_effect(move || {
            let interval = Interval::new(150, move || {
                if*game_over || *is_dying_handler {
                    return;
                }
                let mut new_pos = (*pacman_pos).clone();
                let mut maze_clone = (*maze).clone();
                let mut current_score = *score;

                let can_move = match *current_direction {
                    Direction::Up => {
                        new_pos.y > 0 && maze_clone[new_pos.y - 1][new_pos.x] != 1
                    }
                    Direction::Down => {
                        new_pos.y < maze_clone.len() - 1 && maze_clone[new_pos.y + 1][new_pos.x] != 1
                    }
                    Direction::Left => {
                        new_pos.x > 0 && maze_clone[new_pos.y][new_pos.x - 1] != 1
                    }
                    Direction::Right => {
                        new_pos.x < maze_clone[0].len() - 1 && maze_clone[new_pos.y][new_pos.x + 1] != 1
                    }
                    Direction::None => false,
                };

                if can_move {
                    match *current_direction {
                        Direction::Up => new_pos.y -= 1,
                        Direction::Down => new_pos.y += 1,
                        Direction::Left => new_pos.x -= 1,
                        Direction::Right => new_pos.x += 1,
                        Direction::None => (),
                    }

                    for ghost in (*ghosts).iter() {
                        if ghost.position.x == new_pos.x && ghost.position.y == new_pos.y {
                            is_dying_handler.set(true); 
                            let game_over_clone = game_over.clone();
                            Timeout::new(1000, move || {
                                game_over_clone.set(true);
                            }).forget();
                            return;
                        }
                    }

                    // Check if new position contains food
                    match maze_clone[new_pos.y][new_pos.x] {
                        2 => { // Regular dot
                            current_score += 10;
                            maze_clone[new_pos.y][new_pos.x] = 0;
                        }
                        3 => { // Power pellet
                            current_score += 50;
                            maze_clone[new_pos.y][new_pos.x] = 0;
                        }
                        _ => {}
                    }

                    pacman_pos.set(new_pos);
                    maze.set(maze_clone);
                    score.set(current_score);
                }
            });

            move || drop(interval)
        });
    }

    // Keyboard event listener for direction changes
    {
        let current_direction = current_direction.clone();

        use_effect(move || {
            let document = web_sys::window()
                .unwrap()
                .document()
                .unwrap();

            let handler = move |event: &web_sys::Event| {
                let event = event.dyn_ref::<KeyboardEvent>().unwrap();
                
                let new_direction = match event.key().as_str() {
                    "ArrowUp" => Direction::Up,
                    "ArrowDown" => Direction::Down,
                    "ArrowLeft" => Direction::Left,
                    "ArrowRight" => Direction::Right,
                    _ => return,
                };

                current_direction.set(new_direction);
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
                    .maze-container {
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        padding: 20px;
                        min-height: 100vh;
                        margin-top: auto;
                    }
                    .score {
                        font-size: 32px;
                        font-weight: bold;
                        color: #FFD700;
                        text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
                        padding: 10px 20px;
                        background-color: #000;
                        border: 2px solid #333;
                        border-radius: 10px;
                        margin-bottom: 20px;
                        font-family: 'Arial', sans-serif;
                    }
                    .game-over {
                        position: absolute;
                        top: 50%;
                        left: 50%;
                        transform: translate(-50%, -50%);
                        background-color: rgba(0, 0, 0, 0.8);
                        color: #FF0000;
                        padding: 20px 40px;
                        border-radius: 10px;
                        font-size: 48px;
                        font-weight: bold;
                        text-transform: uppercase;
                        animation: pulse 1.5s infinite;
                        z-index: 1000;
                    }
                    @keyframes pulse {
                        0% { transform: translate(-50%, -50%) scale(1); }
                        50% { transform: translate(-50%, -50%) scale(1.1); }
                        100% { transform: translate(-50%, -50%) scale(1); }
                    }
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
                    .pacman {
                        z-index: 2;
                    }

                    .ghost {
                        z-index: 1;
                    }

                    .pacman.dying {
                        animation: die 1s ease-in-out forwards;
                    }

                    .pacman.dying .pacman-body {
                        animation: die-spin 1s ease-in-out forwards !important;
                    }

                    @keyframes die {
                        0% { transform: scale(1); opacity: 1; }
                        50% { transform: scale(1.5); opacity: 0.5; }
                        100% { transform: scale(0); opacity: 0; }
                    }

                    @keyframes die-spin {
                        0% { transform: rotate(0deg); }
                        100% { transform: rotate(360deg); }
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
            <div class="maze-container">
                <div class="score">{"Score: "}{*score}</div>
                {
                    if *game_over {
                        html! {
                            <div class="game-over">{"Game Over!"}</div>
                        }
                    } else {
                        html! {}
                    }
                }
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
                                            is_dying={*is_dying && is_pacman}
                                        />
                                    }
                            }).collect::<Html>()
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}