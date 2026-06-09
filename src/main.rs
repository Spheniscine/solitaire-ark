use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{BoardComponent, CardComponent}, game::{Board, BoardPos, DepotRole, RANKS, Skin}};

mod game;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");

// altered version of KaTeX_Main to include filled "red" suits
const KATEX_SUITS: Asset = asset!("/assets/KaTeX_Suits.woff2");

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Noto+Emoji:wght@300..700&family=Noto+Sans+Symbols+2&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
            rel: "stylesheet",
        }

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style {
            r#"
            @font-face {{
                font-family: KaTeX_Main;
                font-style: normal;
                font-weight: 700;
                src: url({KATEX_SUITS}) format("woff2");
            }}
            "#,
        }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    let skin = Skin::Animals;
    let mut board = Board::empty();
    board.depots[DepotRole::Tableau.id(0)] = (2..10).collect();
    // board.selected = Some(BoardPos::new(DepotRole::Tableau.id(0), 3));
    rsx! {
        div {
            id: "hero",
            class: "select-none",
            
            BoardComponent { 
                position: Vec2::new(0., 20.),
                board,
                skin,
                num_freecells_unlocked: 2,
            }
        }
    }
}
