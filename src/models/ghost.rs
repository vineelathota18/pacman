use super::Position;
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct Ghost {
    pub position: Position,
    pub color: &'static str,
}

impl Ghost {
    pub fn initialize_ghosts(maze: &[Vec<u8>]) -> Vec<Ghost> {
        let mut rng = rand::thread_rng();
        let ghost_colors = ["#FF0000", "#00FFFF", "#FFB8FF", "#FFB852"];
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
}
