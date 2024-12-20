#[cfg(test)]
mod tests {
    use crate::models::{Direction, Ghost, Position};
    use crate::game_logic::{
        calculate_next_position,
        update_score,
        get_valid_ghost_moves,
    };
    use crate::constants::maze::INITIAL_MAZE;

    #[test]
    fn test_pacman_movement() {
        let maze = INITIAL_MAZE.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        let current_pos = Position { x: 1, y: 1 };

        // Test movement in empty space
        let next_pos = calculate_next_position(&Direction::Right, &current_pos, &maze);
        assert!(next_pos.is_some());
        assert_eq!(next_pos.unwrap(), Position { x: 2, y: 1 });

        // Test wall collision
        let wall_pos = Position { x: 0, y: 0 };
        let next_pos = calculate_next_position(&Direction::Up, &wall_pos, &maze);
        assert!(next_pos.is_none());
    }

    #[test]
    fn test_score_update() {
        let mut maze = vec![
            vec![1, 2, 3],
            vec![0, 2, 0],
            vec![1, 3, 1],
        ];

        // Test regular dot collection
        let pos = Position { x: 1, y: 0 };
        let score = update_score(&pos, &mut maze);
        assert_eq!(score, 10);
        assert_eq!(maze[0][1], 0);

        // Test power pellet collection
        let pos = Position { x: 2, y: 0 };
        let score = update_score(&pos, &mut maze);
        assert_eq!(score, 50);
        assert_eq!(maze[0][2], 0);
    }

    #[test]
    fn test_ghost_moves() {
        let maze = vec![
            vec![1, 0, 0, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![1, 0, 0, 1],
        ];

        let ghost = Ghost {
            position: Position { x: 1, y: 1 },
            color: "#FF0000",
        };

        let valid_moves = get_valid_ghost_moves(&ghost.position, &maze);
        
        // Check possible moves from position (1,1)
        assert!(valid_moves.contains(&Position { x: 1, y: 2 }));  // Down
        assert!(valid_moves.contains(&Position { x: 2, y: 1 }));  // Right
        assert!(valid_moves.contains(&Position { x: 1, y: 0 }));  // Up
        assert!(valid_moves.contains(&Position { x: 0, y: 1 }));  // Left
    }

    #[test]
    fn test_maze_boundaries() {
        let maze = INITIAL_MAZE.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        
        // Test top boundary
        let pos = Position { x: 1, y: 0 };
        let next_pos = calculate_next_position(&Direction::Up, &pos, &maze);
        assert!(next_pos.is_none());

        // Test bottom boundary
        let pos = Position { x: 1, y: maze.len() - 1 };
        let next_pos = calculate_next_position(&Direction::Down, &pos, &maze);
        assert!(next_pos.is_none());
    }
}