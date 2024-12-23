#[cfg(test)]
mod tests {
    use crate::components::cell::CellProps;
    use crate::components::game_board::GameBoardProps;
    use crate::components::scoreboard::{Scoreboard, ScoreboardProps};
    use crate::models::Position;
    use yew::prelude::*;

    #[function_component(TestApp)]
    fn test_app(props: &ScoreboardProps) -> Html {
        html! {
            <Scoreboard
                score={props.score}
                lives={props.lives}
                restart_timer={props.restart_timer}
                game_over={props.game_over}
                game_won={props.game_won}
                game_started={props.game_started}
                on_restart={props.on_restart.clone()}
                on_start={props.on_start.clone()}
            />
        }
    }

    #[test]
    fn test_cell_props_creation() {
        let props = CellProps {
            cell_type: 2,
            is_pacman: false,
            ghost: None,
            is_dying: false,
            is_invincible: false,
            custom_style:None,
        };

        assert_eq!(props.cell_type, 2);
        assert!(!props.is_pacman);
        assert!(props.ghost.is_none());
        assert!(!props.is_dying);
        assert!(!props.is_invincible);
    }

    #[test]
    fn test_game_board_props_creation() {
        let props = GameBoardProps {
            score: 100,
            game_over: false,
            maze: vec![vec![0, 1, 2], vec![2, 3, 0]],
            pacman_pos: Position { x: 0, y: 0 },
            ghosts: vec![],
            is_dying: false,
            is_invincible: false,
        };

        assert_eq!(props.score, 100);
        assert!(!props.game_over);
        assert_eq!(props.maze.len(), 2);
        assert_eq!(props.maze[0].len(), 3);
        assert_eq!(props.pacman_pos.x, 0);
        assert_eq!(props.pacman_pos.y, 0);
        assert!(props.ghosts.is_empty());
        assert!(!props.is_dying);
        assert!(!props.is_invincible);
    }
}
