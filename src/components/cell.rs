use crate::models::Ghost;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub cell_type: u8,
    pub is_pacman: bool,
    pub ghost: Option<Ghost>,
    pub is_dying: bool,
    pub is_invincible: bool,
}

#[function_component]
pub fn Cell(props: &CellProps) -> Html {
    let class = classes!(
        "cell",
        if props.is_pacman {
            if props.is_dying {
                "pacman dying"
            } else {
                "pacman"
            }
        } else if props.ghost.is_some() {
            "ghost"
        } else {
            match props.cell_type {
                1 => "wall",
                2 => "dot",
                3 => "power-pellet",
                _ => "empty",
            }
        }
    );

    let content = if props.is_pacman {
        html! {
            <>
            <div class="pacman-body"></div>
            <div class="pacman-eye"></div>
            </>
        }
    } else if let Some(ghost) = &props.ghost {
        let ghost_color = if props.is_invincible {
            "#808080".to_string() // Grey color for vulnerable ghosts
        } else {
            ghost.color.to_string()
        };
        let style = format!("background-color: {};", ghost_color);
        html! {
            <div class="ghost-body" {style}>{"👻"}</div>
        }
    } else {
        match props.cell_type {
            2 => html! { "." },
            3 => html! { "⚪" },
            _ => html! { "" },
        }
    };

    html! {
        <div class={class}>
            {content}
        </div>
    }
}
