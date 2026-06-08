use dioxus::prelude::*;

use crate::{components::{EMOJI_MAP, Emoji, SkinTrait}, game::{Card, ColorMode, Skin}};

pub const KATEX_MAIN: &str = "KaTeX_Main";
const COLOR_BLACK: [&str; 2] = ["#000", "#fff"];

impl SkinTrait<Card> for Skin {
    fn get_color(&self, _card: &Card, mode: ColorMode) -> String {
        COLOR_BLACK[mode as usize].to_string()
    }

    fn render_rank(&self, card: &Card) -> Element {
        match self {
            Skin::Animals => rsx! {
                Emoji {
                    text: *EMOJI_MAP.index(*card as usize).unwrap().0,
                }
            },
            Skin::Numbers => rsx! {
                span {
                    font_family: KATEX_MAIN,
                    "{card + 1}",
                }
            },
        }
    }

    fn render_suit(&self, _card: &Card) -> Element {
        rsx! {}
    }
}