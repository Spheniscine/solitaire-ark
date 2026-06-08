use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::CardComponent, game::{RANKS, Skin}};

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
    rsx! {
        div {
            id: "hero",
            
            for card in RANKS {
                CardComponent { 
                    position: Vec2::new(40., 14. * card as f32),
                    width: 13.,
                    card,
                    skin,
                }
            }
        }
    }
}
