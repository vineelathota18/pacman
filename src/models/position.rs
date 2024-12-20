use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}