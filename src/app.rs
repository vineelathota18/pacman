use crate::controls;
use crate::game_logic;
use crate::models::Ghost;
use crate::components::game_board::GameBoard;
use crate::components::scoreboard::Scoreboard;
use crate::constants::maze::INITIAL_MAZE;
use crate::models::{Direction, Position};
use yew::prelude::*;
use yew_hooks::use_counter;
use gloo::timers::callback::{Interval, Timeout};

#[function_component]
pub fn App() -> Html {
    let maze = use_state(|| {
        INITIAL_MAZE
            .iter()
            .map(|row| row.to_vec())
            .collect::<Vec<_>>()
    });
    let pacman_pos = use_state(|| Position { x: 7, y: 6 });
    let score = use_state(|| 0);
    let is_dying = use_state(|| false);
    let current_direction = use_state(|| Direction::None);
    let game_over = use_state(|| false);
    let move_counter = use_state(|| 0);
    let is_invincible = use_state(|| false);
    let lives = use_state(|| 3);
    let restart_timer = use_state(|| false);
    let initial_ghost_positions = use_state(|| Ghost::initialize_ghosts(&maze));
    let ghosts = use_state(|| (*initial_ghost_positions).clone());
    let invincibility = use_counter(0);
    let game_won: UseStateHandle<bool> = use_state(|| false);
    let game_started = use_state(|| false);

    let start_game = {
        let game_started = game_started.clone();
        Callback::from(move |_: MouseEvent| {  
            game_started.set(true);
        })
    };

    let reset_positions = {
        let pacman_pos = pacman_pos.clone();
        let ghosts = ghosts.clone();
        let initial_ghost_positions = initial_ghost_positions.clone();
        let is_dying = is_dying.clone();
        let current_direction = current_direction.clone();
        let restart_timer = restart_timer.clone();
        let lives = lives.clone();
        let game_over = game_over.clone();
        let game_started = game_started.clone();


        move || {
            if *lives > 1 {
                restart_timer.set(true);
                let timer_handle = Timeout::new(3000, move || {
                    pacman_pos.set(Position { x: 7, y: 6 });
                    ghosts.set((*initial_ghost_positions).clone());
                    is_dying.set(false);
                    current_direction.set(Direction::None);
                    restart_timer.set(false);
                    game_started.set(true);
                });
                timer_handle.forget();
            } else {
                game_over.set(true);
            }
        }
    };

    let restart_game = {
        let maze = maze.clone();
        let pacman_pos = pacman_pos.clone();
        let score = score.clone();
        let is_dying = is_dying.clone();
        let current_direction = current_direction.clone();
        let game_over = game_over.clone();
        let move_counter = move_counter.clone();
        let lives = lives.clone();
        let ghosts = ghosts.clone();
        let is_invincible = is_invincible.clone();
        let restart_timer = restart_timer.clone();
        let game_won: UseStateHandle<bool> = game_won.clone();
        let game_started = game_started.clone();

        Callback::from(move |_: MouseEvent| {
            maze.set(
                INITIAL_MAZE
                    .iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>(),
            );
            game_started.set(true);
            pacman_pos.set(Position { x: 7, y: 6 });
            score.set(0);
            is_dying.set(false);
            current_direction.set(Direction::None);
            game_over.set(false);
            move_counter.set(0);
            lives.set(3);
            is_invincible.set(false);
            restart_timer.set(false);
            let initial_ghosts = Ghost::initialize_ghosts(&maze);
            ghosts.set(initial_ghosts);
            game_won.set(false);
        })
    };

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
            lives,
            restart_timer,
            invincibility,
            game_won,
            game_started
        ) = (
            pacman_pos.clone(),
            current_direction.clone(),
            maze.clone(),
            score.clone(),
            game_over.clone(),
            ghosts.clone(),
            is_dying.clone(),
            move_counter.clone(),
            lives.clone(),
            restart_timer.clone(),
            invincibility.clone(),
            game_won.clone(),
            game_started.clone(),
        );

        use_effect(move || {
            let interval = Interval::new(150, move || {
                if !*game_started || *game_over || *is_dying || *restart_timer {
                    return;
                }

                let new_counter = *move_counter + 1;
                move_counter.set(new_counter);

                if game_logic::check_ghost_collision(
                    &pacman_pos,
                    &ghosts,
                    is_dying.clone(),
                    lives.clone(),
                    *invincibility,
                ) {
                    let reset = reset_positions.clone();
                    Timeout::new(1000, move || {
                        reset();
                    })
                    .forget();
                    return;
                }

                if new_counter % 2 == 0 {
                    let mut new_ghosts = (*ghosts).clone();
                    game_logic::move_ghosts(&mut new_ghosts, &pacman_pos, &maze);
                    ghosts.set(new_ghosts);
                }

                let mut new_pos = (*pacman_pos).clone();
                let mut maze_clone = (*maze).clone();
                let mut current_score = *score;

                if game_logic::check_game_complete(&maze_clone) {
                    game_won.set(true);
                    return;
                }

                if let Some((next_pos, power_pellet_eaten)) = game_logic::calculate_next_position(
                    &current_direction,
                    &new_pos,
                    &mut maze_clone,
                    &mut current_score,
                ) {
                    new_pos = next_pos;

                    if power_pellet_eaten {
                        invincibility.increase();
                        let invincibility_clone = invincibility.clone();
                        Timeout::new(5000, move || {
                            invincibility_clone.decrease();
                        })
                        .forget();
                    }

                    pacman_pos.set(new_pos);
                    maze.set(maze_clone);
                    score.set(current_score);
                }
            });

            move || drop(interval)
        });
    }

    {
        let current_direction = current_direction.clone();
        use_effect(move || {
            let listener = controls::setup_keyboard_controls(current_direction);
            move || drop(listener)
        });
    }

    html! {
        <>
            <Scoreboard
                score={*score}
                lives={*lives}
                restart_timer={*restart_timer}
                game_over={*game_over}
                on_restart={restart_game.clone()}
                game_won={*game_won}
                game_started={*game_started}
                on_start={start_game.clone()}
            />
            <GameBoard
                score={*score}
                game_over={*game_over}
                maze={(*maze).clone()}
                pacman_pos={(*pacman_pos).clone()}
                ghosts={(*ghosts).clone()}
                is_dying={*is_dying}
                is_invincible={*invincibility > 0}
            />
        </>
    }
}
