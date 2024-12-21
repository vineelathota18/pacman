use super::Position;
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct Ghost {
    pub position: Position,
    pub color: &'static str,
}

impl Ghost {
    pub fn initialize_ghosts(_maze: &[Vec<u8>]) -> Vec<Ghost> {
        // Hard-coded center positions for ghosts
        vec![
            Ghost {
                position: Position { x: 14, y: 11 },  // Red ghost in center
                color: "#FF0000"
            },
            Ghost {
                position: Position { x: 14, y: 12 },  // Cyan ghost
                color: "#00FFFF"
            },
            Ghost {
                position: Position { x: 15, y: 11 },  // Pink ghost
                color: "#FFB8FF"
            },
            Ghost {
            position: Position { x: 15, y: 12 },  // Orange ghost
                color: "#FFB852"
            }
        ]
    }
}
