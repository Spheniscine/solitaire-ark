use dioxus::prelude::*;

use crate::{components::{Emph, VIDEO_GAMEPLAY, rem}, game::{GameState, ScreenState}};


#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    // let st = game_state.read();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 4.5rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The ",Emph {"tableau"}," consists of 8 rows. Matching cards can be stacked in the tableau and moved as a group."
                }

                p {
                    "When four matching cards are united on the tableau, they collapse into a ",Emph {"locked stack"},". If there are locked free cells, 
                    this will unlock one of them."
                }

                p {
                    Emph {"Free cells"}," may store a single card of any kind. Four matching cards can also be united in a free cell to collapse
                    them into a locked stack."
                }

                p {
                    "To ",Emph {"win the game"},", collapse all ten kinds of cards into ten locked stacks."
                }

                p {
                    Emph{"Shortcut note:"}," Double-clicking on a card will automatically try to move it to a free cell. This includes stacks that would form a full set."
                }

                div {
                    position: "absolute",
                    bottom: rem(2.),
                    width: "92rem",
                    display: "flex",
                    justify_content: "center",

                    a {
                        href: VIDEO_GAMEPLAY,
                        target: "_blank",
                        text_decoration: "none",
                        margin_right: rem(4.),
                        div {
                            width: rem(30.),
                            position: "relative",
                            class: "game-button",
                            "Example video"
                        }
                    }

                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                        "Back to game"
                    }
                }
                
            }
        }
        
    }
}