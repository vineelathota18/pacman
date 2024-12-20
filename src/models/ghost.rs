use super::Position;

#[derive(Clone, PartialEq)]
pub struct Ghost {
    pub position: Position,
    pub color: &'static str,
}

impl Ghost {
    pub fn get_possible_moves(&self, maze: &Vec<Vec<u8>>) -> Vec<Position> {
        let mut moves = Vec::new();
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for (dx, dy) in directions.iter() {
            let new_x = self.position.x as i32 + dx;
            let new_y = self.position.y as i32 + dy;

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

    pub fn move_towards_pacman(&mut self, pacman_pos: &Position, maze: &Vec<Vec<u8>>) {
        let possible_moves = self.get_possible_moves(maze);
        if possible_moves.is_empty() {
            return;
        }

        let best_move = possible_moves
            .iter()
            .min_by_key(|pos| {
                let dx = pos.x as i32 - pacman_pos.x as i32;
                let dy = pos.y as i32 - pacman_pos.y as i32;
                dx * dx + dy * dy
            })
            .unwrap()
            .clone();

        self.position = best_move;
    }
}
