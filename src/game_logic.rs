use crate::models::{Direction, Ghost, Position};
use gloo::timers::callback::Timeout;
use rand::Rng;
use yew::UseStateHandle;

/// Calculate valid moves for ghosts based on the maze layout
pub fn get_valid_ghost_moves(position: &Position, maze: &[Vec<u8>]) -> Vec<Position> {
    let mut moves = Vec::new();
    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // Up, Down, Left, Right
    
    for (dx, dy) in directions.iter() {
        let new_x = position.x as i32 + dx;
        let new_y = position.y as i32 + dy;
        
        if new_x >= 0 && new_y >= 0 && 
           new_x < maze[0].len() as i32 && 
           new_y < maze.len() as i32 && 
           maze[new_y as usize][new_x as usize] != 1 {
            moves.push(Position {
                x: new_x as usize,
                y: new_y as usize,
            });
        }
    }
    moves
}

/// Initialize ghost positions in valid locations
pub fn initialize_ghosts(maze: &[Vec<u8>]) -> Vec<Ghost> {
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

/// Calculate the next position for Pacman based on current direction
pub fn calculate_next_position(
    current_direction: &Direction,
    current_pos: &Position,
    maze: &[Vec<u8>],
) -> Option<Position> {
    let mut new_pos = current_pos.clone();

    let can_move = match current_direction {
        Direction::Up => {
            new_pos.y > 0 && maze[new_pos.y - 1][new_pos.x] != 1
        }
        Direction::Down => {
            new_pos.y < maze.len() - 1 && maze[new_pos.y + 1][new_pos.x] != 1
        }
        Direction::Left => {
            new_pos.x > 0 && maze[new_pos.y][new_pos.x - 1] != 1
        }
        Direction::Right => {
            new_pos.x < maze[0].len() - 1 && maze[new_pos.y][new_pos.x + 1] != 1
        }
        Direction::None => false,
    };

    if can_move {
        match current_direction {
            Direction::Up => new_pos.y -= 1,
            Direction::Down => new_pos.y += 1,
            Direction::Left => new_pos.x -= 1,
            Direction::Right => new_pos.x += 1,
            Direction::None => (),
        }
        Some(new_pos)
    } else {
        None
    }
}

/// Update game score based on collected items
pub fn update_score(pos: &Position, maze: &mut Vec<Vec<u8>>) -> i32 {
    match maze[pos.y][pos.x] {
        2 => { // Regular dot
            maze[pos.y][pos.x] = 0;
            10
        }
        3 => { // Power pellet
            maze[pos.y][pos.x] = 0;
            50
        }
        _ => 0,
    }
}

/// Check for collisions between Pacman and ghosts
pub fn check_ghost_collision(
    pacman_pos: &Position,
    ghosts: &[Ghost],
    is_dying: UseStateHandle<bool>,
    game_over: UseStateHandle<bool>,
) -> bool {
    for ghost in ghosts {
        if ghost.position == *pacman_pos {
            is_dying.set(true);
            let game_over_clone = game_over.clone();
            Timeout::new(1000, move || {
                game_over_clone.set(true);
            })
            .forget();
            return true;
        }
    }
    false
}

/// Move ghosts towards Pacman
pub fn move_ghosts(ghosts: &mut [Ghost], pacman_pos: &Position, maze: &[Vec<u8>]) {
    for ghost in ghosts.iter_mut() {
        let possible_moves = get_valid_ghost_moves(&ghost.position, maze);
        if let Some(best_move) = find_best_move(&possible_moves, pacman_pos) {
            ghost.position = best_move;
        }
    }
}

/// Find the best move for a ghost to reach Pacman
fn find_best_move(possible_moves: &[Position], target: &Position) -> Option<Position> {
    possible_moves.iter()
        .min_by_key(|pos| {
            let dx = pos.x as i32 - target.x as i32;
            let dy = pos.y as i32 - target.y as i32;
            dx * dx + dy * dy // Manhattan distance
        })
        .cloned()
}

/// Check if the game is complete (all dots collected)
pub fn check_game_complete(maze: &[Vec<u8>]) -> bool {
    !maze.iter().any(|row| row.iter().any(|&cell| cell == 2 || cell == 3))
}