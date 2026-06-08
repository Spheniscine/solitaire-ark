use dioxus::{logger::tracing, prelude::*};
use phf::phf_ordered_map;

/// First 10 slots reserved for the animals that represent ranks
pub static EMOJI_MAP: phf::OrderedMap<&'static str, Asset> = phf_ordered_map! {
    "🐰" => asset!("/assets/emoji/emoji_u1f430.svg"),
    "🦊" => asset!("/assets/emoji/emoji_u1f98a.svg"),
    "🐯" => asset!("/assets/emoji/emoji_u1f42f.svg"),
    "🐧" => asset!("/assets/emoji/emoji_u1f427.svg"),
    "🐼" => asset!("/assets/emoji/emoji_u1f43c.svg"),
    "🕊️" => asset!("/assets/emoji/emoji_u1f54a.svg"),
    "🐺" => asset!("/assets/emoji/emoji_u1f43a.svg"),
    "🐘" => asset!("/assets/emoji/emoji_u1f418.svg"),
    "🦖" => asset!("/assets/emoji/emoji_u1f996.svg"),
    "🦔" => asset!("/assets/emoji/emoji_u1f994.svg"),
};

#[component]
pub fn Emoji(text: String) -> Element {
    if let Some(asset) = EMOJI_MAP.get(&text) {
        rsx! {
            img {
                style: "height: 1.125em; vertical-align: middle;",
                src: *asset,
                draggable: false,
                alt: text,
            }
        }
    } else {
        tracing::error!("No emoji asset loaded for string '{text}'");
        rsx! {
            "ERROR"
        }
    }
    
}