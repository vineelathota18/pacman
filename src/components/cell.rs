use yew::prelude::*;
use crate::models::Ghost;

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub cell_type: u8,
    pub is_pacman: bool,
    pub ghost: Option<Ghost>,
    pub is_dying: bool,
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
        let style = format!("background-color: {};", ghost.color);
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