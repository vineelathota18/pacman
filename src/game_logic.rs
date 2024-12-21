use crate::models::{Direction, Ghost, Position};
use rand::Rng;
use yew::UseStateHandle;

pub fn find_ghost_move(
    ghost: &Ghost,
    pacman_pos: &Position,
    maze: &[Vec<u8>],
    aggressive: bool,
) -> Option<Position> {
    let possible_moves = get_valid_ghost_moves(&ghost.position, maze);
    if possible_moves.is_empty() {
        return None;
    }

    // Randomly decide between best and worst move
    let mut rng = rand::thread_rng();
    let make_best_move = if aggressive {
        true
    } else {
        rng.gen_bool(0.7) // 70% chance to make best move
    };

    // Find the move that gets us closest to or furthest from Pacman
    find_best_move(&possible_moves, pacman_pos, make_best_move)
}

/// Calculate valid moves for ghosts based on the maze layout
pub fn get_valid_ghost_moves(position: &Position, maze: &[Vec<u8>]) -> Vec<Position> {
    let mut moves = Vec::new();
    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // Up, Down, Left, Right

    for (dx, dy) in directions.iter() {
        let new_x = position.x as i32 + dx;
        let new_y = position.y as i32 + dy;

        if new_x >= 0
            && new_y >= 0
            && new_x < maze[0].len() as i32
            && new_y < maze.len() as i32
            && maze[new_y as usize][new_x as usize] != 1
        {
            moves.push(Position {
                x: new_x as usize,
                y: new_y as usize,
            });
        }
    }
    moves
}

/// Find the best move for a ghost to reach Pacman
fn find_best_move(
    possible_moves: &[Position],
    pacman_pos: &Position,
    make_best_move: bool,
) -> Option<Position> {
    possible_moves
        .iter()
        .min_by_key(|pos| {
            let dx = pos.x as i32 - pacman_pos.x as i32;
            let dy = pos.y as i32 - pacman_pos.y as i32;
            let distance = dx * dx + dy * dy;
            if make_best_move {
                distance
            } else {
                -distance
            }
        })
        .cloned()
}

/// Calculate the next position for Pacman based on current direction
pub fn calculate_next_position(
    current_direction: &Direction,
    current_pos: &Position,
    maze: &mut [Vec<u8>],
    score: &mut i32,
) -> Option<(Position, bool)> {
    let mut new_pos = current_pos.clone();

    let can_move = match current_direction {
        Direction::Up => new_pos.y > 0 && maze[new_pos.y - 1][new_pos.x] != 1,
        Direction::Down => new_pos.y < maze.len() - 1 && maze[new_pos.y + 1][new_pos.x] != 1,
        Direction::Left => new_pos.x > 0 && maze[new_pos.y][new_pos.x - 1] != 1,
        Direction::Right => new_pos.x < maze[0].len() - 1 && maze[new_pos.y][new_pos.x + 1] != 1,
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

        let power_pellet_eaten = update_score(&new_pos, maze, score);
        Some((new_pos, power_pellet_eaten))
    } else {
        None
    }
}

// Update game score based on collected items
// Returns true if power pellet was eaten
pub fn update_score(pos: &Position, maze: &mut [Vec<u8>], score: &mut i32) -> bool {
    match maze[pos.y][pos.x] {
        2 => {
            *score += 10;
            maze[pos.y][pos.x] = 0;
            false
        }
        3 => {
            *score += 50;
            maze[pos.y][pos.x] = 0;
            true
        }
        _ => false,
    }
}

/// Check for collisions between Pacman and ghosts
pub fn check_ghost_collision(
    pacman_pos: &Position,
    ghosts: &[Ghost],
    is_dying: UseStateHandle<bool>,
    lives: UseStateHandle<i32>,
    invincibility: i32,
) -> bool {
    if invincibility <= 0 {
        for ghost in ghosts {
            if ghost.position == *pacman_pos {
                is_dying.set(true);
                lives.set(*lives - 1);
                return true;
            }
        }
    }
    false
}

/// Move ghosts towards Pacman
pub fn move_ghosts(ghosts: &mut [Ghost], pacman_pos: &Position, maze: &[Vec<u8>]) {
    let mut rng = rand::thread_rng();

    for ghost in ghosts.iter_mut() {
        // Each ghost has a different personality
        let aggressive = match ghost.color {
            "#FF0000" => true,
            "#00FFFF" => rng.gen_bool(0.4),
            "#FFB8FF" => rng.gen_bool(0.3),
            "#FFB852" => false,
            _ => rng.gen_bool(0.7),
        };

        if let Some(new_pos) = find_ghost_move(ghost, pacman_pos, maze, aggressive) {
            ghost.position = new_pos;
        }
    }
}

pub fn check_game_complete(maze: &[Vec<u8>]) -> bool {
    !maze
        .iter()
        .any(|row| row.iter().any(|&cell| cell == 2 || cell == 3))
}
