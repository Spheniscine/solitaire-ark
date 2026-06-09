use dioxus::prelude::*;

use crate::{components::{Emph, rem}, game::{Difficulty, GameState, ScreenState}};

#[component]
fn Choice(
    name: String,
    description: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            style: "text-align: center; border: 0.5rem solid #00B163; border-radius: 2rem; padding: 2rem; margin-top: 7rem;",
            onclick,
            {name},
            br {},
            div {
                font_size: rem(3.5),
                {description},
            }
        }
    }
}

#[component]
pub fn NewGame(game_state: Signal<GameState>) -> Element {
    let max_difficulty = game_state.read().max_difficulty;
    rsx! {
        div {
            font_size: rem(6.),
            text_align: "center",
            color: "#fff",
            margin: rem(4.),

            p {
                margin_top: rem(10.),

                "Welcome to Ark Solitaire!"
                br {}
                "Please choose a difficulty:",
            }

            Choice {
                name: "Easy",
                description: "All 4 free cells begin unlocked.",
                onclick: move |_| game_state.write().new_game_with_difficulty(Difficulty::Easy),
            }

            if max_difficulty >= Difficulty::Medium {
                Choice {
                    name: "Medium",
                    description: "3 of 4 free cells begin unlocked, 1 begins locked.",
                    onclick: move |_| game_state.write().new_game_with_difficulty(Difficulty::Medium),
                }
            }

            if max_difficulty >= Difficulty::Hard {
                Choice {
                    name: "Hard",
                    description: "2 of 4 free cells begin unlocked, 2 begin locked.",
                    onclick: move |_| game_state.write().new_game_with_difficulty(Difficulty::Hard),
                }
            }

            if max_difficulty >= Difficulty::Expert {
                Choice {
                    name: "Expert",
                    description: "Only 1 of 4 free cells begins unlocked, 3 begin locked.",
                    onclick: move |_| game_state.write().new_game_with_difficulty(Difficulty::Expert),
                }
            }

            if max_difficulty < Difficulty::Expert {
                p {
                    margin_top: rem(7.),
                    font_size: rem(4.),
                    "Beat the ",Emph {"{max_difficulty}"}," difficulty to unlock ",Emph {"{max_difficulty.next_up()}"}," difficulty! Or unlock them all in the Settings screen."
                }
            }

            button {
                margin_top: rem(10.),
                onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                "Cancel",
            }
            
        }
        
    }
}