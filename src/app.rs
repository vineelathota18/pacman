use crate::components::game_board::GameBoard;
use crate::constants::maze::INITIAL_MAZE;
use crate::models::{Direction, Ghost, Position};
use gloo::events::EventListener;
use gloo::timers::callback::{Interval, Timeout};
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::prelude::*;

fn initialize_ghosts(maze: &Vec<Vec<u8>>) -> Vec<Ghost> {
    let mut rng = rand::thread_rng();
    let ghost_colors = vec!["#FF0000", "#00FFFF", "#FFB8FF", "#FFB852"];
    let mut valid_positions: Vec<Position> = maze
        .iter()
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

    ghost_colors
        .iter()
        .filter_map(|&color| {
            if valid_positions.is_empty() {
                None
            } else {
                let idx = rng.gen_range(0..valid_positions.len());
                let position = valid_positions.remove(idx);
                Some(Ghost { position, color })
            }
        })
        .collect()
}

#[function_component]
pub fn App() -> Html {
    // State initialization
    let maze = use_state(|| INITIAL_MAZE.iter().map(|row| row.to_vec()).collect::<Vec<_>>());
    let pacman_pos = use_state(|| Position { x: 1, y: 1 });
    let score = use_state(|| 0);
    let is_dying = use_state(|| false);
    let current_direction = use_state(|| Direction::None);
    let game_over = use_state(|| false);
    let move_counter = use_state(|| 0);
    let ghosts = use_state(|| initialize_ghosts(&maze));

    // Game loop effect
    {
        let (
            pacman_pos,
            current_direction,
            maze,
            score,
            game_over,
            ghosts,
            is_dying,
            move_counter,
        ) = (
            pacman_pos.clone(),
            current_direction.clone(),
            maze.clone(),
            score.clone(),
            game_over.clone(),
            ghosts.clone(),
            is_dying.clone(),
            move_counter.clone(),
        );

        use_effect(move || {
            let interval = Interval::new(150, move || {
                if *game_over || *is_dying {
                    return;
                }

                // Update move counter and move ghosts
                let new_counter = *move_counter + 1;
                move_counter.set(new_counter);

                // Ghost movement logic
                if new_counter % 2 == 0 {
                    let mut new_ghosts = (*ghosts).clone();
                    for ghost in new_ghosts.iter_mut() {
                        ghost.move_towards_pacman(&pacman_pos, &maze);
                    }
                    ghosts.set(new_ghosts);
                }

                // Check for ghost collision
                for ghost in (*ghosts).iter() {
                    if ghost.position == *pacman_pos {
                        is_dying.set(true);
                        let game_over_clone = game_over.clone();
                        Timeout::new(1000, move || {
                            game_over_clone.set(true);
                        })
                        .forget();
                        return;
                    }
                }

                // Pacman movement
                let mut new_pos = (*pacman_pos).clone();
                let mut maze_clone = (*maze).clone();
                let mut current_score = *score;

                let can_move = match *current_direction {
                    Direction::Up => new_pos.y > 0 && maze_clone[new_pos.y - 1][new_pos.x] != 1,
                    Direction::Down => {
                        new_pos.y < maze_clone.len() - 1 && maze_clone[new_pos.y + 1][new_pos.x] != 1
                    }
                    Direction::Left => new_pos.x > 0 && maze_clone[new_pos.y][new_pos.x - 1] != 1,
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

                    // Update score based on collected items
                    match maze_clone[new_pos.y][new_pos.x] {
                        2 => {
                            current_score += 10;
                            maze_clone[new_pos.y][new_pos.x] = 0;
                        }
                        3 => {
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

    // Keyboard controls effect
    {
        let current_direction = current_direction.clone();

        use_effect(move || {
            let document = web_sys::window().unwrap().document().unwrap();

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

            move || drop(listener)
        });
    }

    // Render game board
    html! {
        <GameBoard
            score={*score}
            game_over={*game_over}
            maze={(*maze).clone()}
            pacman_pos={(*pacman_pos).clone()}
            ghosts={(*ghosts).clone()}
            is_dying={*is_dying}
        />
    }
}