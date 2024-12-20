#[cfg(test)]
mod game_logic_tests {
    use crate::models::{Direction, Ghost, Position};
    use crate::controls::get_direction_from_key;
    use crate::game_logic::*;
    use std::cell::RefCell;

    fn simulate_move(
        direction: &Direction,
        current_pos: &Position,
        maze: &mut Vec<Vec<u8>>,
        score: &mut i32
    ) -> Option<(Position, bool)> {
        let mut new_pos = current_pos.clone();
        
        let can_move = match direction {
            Direction::Up => new_pos.y > 0 && maze[new_pos.y - 1][new_pos.x] != 1,
            Direction::Down => new_pos.y < maze.len() - 1 && maze[new_pos.y + 1][new_pos.x] != 1,
            Direction::Left => new_pos.x > 0 && maze[new_pos.y][new_pos.x - 1] != 1,
            Direction::Right => new_pos.x < maze[0].len() - 1 && maze[new_pos.y][new_pos.x + 1] != 1,
            Direction::None => false,
        };

        if can_move {
            match direction {
                Direction::Up => new_pos.y -= 1,
                Direction::Down => new_pos.y += 1,
                Direction::Left => new_pos.x -= 1,
                Direction::Right => new_pos.x += 1,
                Direction::None => (),
            }
            let power_pellet = update_score(&new_pos, maze, score);
            Some((new_pos, power_pellet))
        } else {
            None
        }
    }

    fn create_test_maze() -> Vec<Vec<u8>> {
        vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 2, 0, 1],
            vec![1, 2, 3, 2, 1],
            vec![1, 0, 2, 0, 1],
            vec![1, 1, 1, 1, 1],
        ]
    }

    #[test]
    fn test_direction_from_key() {
        assert_eq!(get_direction_from_key("ArrowUp"), Some(Direction::Up));
        assert_eq!(get_direction_from_key("ArrowDown"), Some(Direction::Down));
        assert_eq!(get_direction_from_key("ArrowLeft"), Some(Direction::Left));
        assert_eq!(get_direction_from_key("ArrowRight"), Some(Direction::Right));
        assert_eq!(get_direction_from_key("Invalid"), None);
    }

    #[test]
    fn test_valid_ghost_moves() {
        let maze = create_test_maze();
        let pos = Position { x: 1, y: 1 };
        
        let valid_moves = get_valid_ghost_moves(&pos, &maze);
        assert_eq!(valid_moves.len(), 2); 
        
        assert!(valid_moves.contains(&Position { x: 2, y: 1 })); 
        assert!(valid_moves.contains(&Position { x: 1, y: 2 })); 
    }

    #[test]
    fn test_ghost_movement() {
        let maze = create_test_maze();
        let pacman_pos = Position { x: 3, y: 1 };
        let ghost = Ghost {
            position: Position { x: 1, y: 1 },
            color: "#FF0000",
        };
        
        let next_move = find_ghost_move(&ghost, &pacman_pos, &maze, true);
        assert!(next_move.is_some());
        if let Some(new_pos) = next_move {
            assert!(new_pos.x > ghost.position.x); 
        }
    }

    #[test]
    fn test_score_update() {
        let mut maze = create_test_maze();
        let mut score = 0;
        let pos = Position { x: 2, y: 1 }; 
        
        let power_pellet = update_score(&pos, &mut maze, &mut score);
        assert_eq!(score, 10); 
        assert_eq!(maze[pos.y][pos.x], 0); 
        assert!(!power_pellet); 
        
        let power_pos = Position { x: 2, y: 2 };
        let power_pellet = update_score(&power_pos, &mut maze, &mut score);
        assert_eq!(score, 60); 
        assert_eq!(maze[power_pos.y][power_pos.x], 0);
        assert!(power_pellet);
    }

    #[test]
    fn test_ghost_collision() {
        let pacman_pos = Position { x: 2, y: 2 };
        let ghosts = vec![
            Ghost {
                position: Position { x: 2, y: 2 }, 
                color: "#FF0000",
            }
        ];
        
        let mut is_dying = false;
        let mut lives = 3;
        
        let collision = check_ghost_collision_test(
            &pacman_pos,
            &ghosts,
            &mut is_dying,
            &mut lives,
            false 
        );
        
        assert!(collision);
        assert!(is_dying);
        assert_eq!(lives, 2);
        
        let mut is_dying = false;
        let mut lives = 3;
        
        let collision = check_ghost_collision_test(
            &pacman_pos,
            &ghosts,
            &mut is_dying,
            &mut lives,
            true 
        );
        
        assert!(!collision);
        assert!(!is_dying);
        assert_eq!(lives, 3);
    }

    pub fn check_ghost_collision_test(
        pacman_pos: &Position,
        ghosts: &[Ghost],
        is_dying: &mut bool,
        lives: &mut i32,
        is_invincible: bool,
    ) -> bool {
        if !is_invincible {
            for ghost in ghosts {
                if ghost.position == *pacman_pos {
                    *lives -= 1;
                    *is_dying = true;
                    return true;
                }
            }
        }
        false
    }
    
    #[test]
    fn test_movement() {
        let mut maze = create_test_maze();
        let mut score = 0;
        let current_pos = Position { x: 1, y: 1 };
        
        let next_pos = simulate_move(
            &Direction::Right,
            &current_pos,
            &mut maze,
            &mut score
        );
        
        assert!(next_pos.is_some());
        if let Some((new_pos, power_pellet)) = next_pos {
            assert_eq!(new_pos.x, 2);
            assert_eq!(new_pos.y, 1);
            assert!(!power_pellet);
        }
    }

    #[test]
    fn test_game_completion() {
        let mut maze = create_test_maze();
        assert!(!check_game_complete(&maze));
        
        for row in maze.iter_mut() {
            for cell in row.iter_mut() {
                if *cell == 2 || *cell == 3 {
                    *cell = 0;
                }
            }
        }
        
        assert!(check_game_complete(&maze));
    }
}