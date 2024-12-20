use crate::models::direction;

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Ghost {
    pub position: Position,
    pub color: &'static str,
}
