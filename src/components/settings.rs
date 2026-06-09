use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::{components::rem, game::{GameState, ScreenState, Skin}};

#[component]
pub fn Settings(game_state: Signal<GameState>) -> Element {
    let mut state = use_signal(|| {
        game_state.read().new_settings_state()
    });
    let mut ok = move || {
        game_state.write().apply_settings(&state.read());
        game_state.write().screen_state = ScreenState::Game;
    };
    let mut cancel = move || {
        game_state.write().screen_state = ScreenState::Game;
    };

    let onmounted = async move |e: Event<MountedData>| {
        let _ = e.set_focus(true).await;
    };
    let onkeydown = move |e: Event<KeyboardData>| {
        let key = e.key();
        match key {
            Key::Enter => {
                ok();
            }
            Key::Escape => {
                cancel();
            }
            _ => {}
        }
    };

    let allow_undo_changed = move |evt: Event<FormData>| {
        state.write().allow_undo = evt.checked();
    };

    let unlock_difficulties_changed = move |evt: Event<FormData>| {
        state.write().unlock_difficulties = evt.checked();
    };

    let skin_changed = move |evt: Event<FormData>| {
        let v = evt.value().parse().ok().and_then(|v| { Skin::from_repr(v) });
        state.write().skin = v.unwrap_or_default();
    };

    rsx! {
        div {
            id: "settingsDialog",
            tabindex: -1,
            onmounted: onmounted,
            onkeydown: onkeydown,

            p {
                "Allow undo/reset: "
                input {
                    r#type: "checkbox",
                    checked: state.read().allow_undo,
                    onchange: allow_undo_changed,
                }
            }

            p {
                line_height: 1,
                "Unlock all difficulties: "
                input {
                    r#type: "checkbox",
                    checked: state.read().unlock_difficulties,
                    onchange: unlock_difficulties_changed,
                    disabled: state.read().unlock_difficulties_disabled,
                }
                br {}
                span {
                    font_size: rem(3.5),
                    "(Cannot be undone)"
                }
            }

            p {
                "Card style: "
                select {
                    onchange: skin_changed,
                    for x in Skin::iter() {
                        option {
                            value: x as usize,
                            selected: state.read().skin == x,
                            "{x}"
                        }
                    }
                }
            }

            p {
                button {
                    r#type: "button",
                    onclick: move |_| ok(),
                    "OK"
                }
                " ",
                button {
                    r#type: "button",
                    onclick: move |_| cancel(),
                    "Cancel"
                }
            }

            p {
                class: "copyright",
                "Game rules: “Kabufuda Solitaire” by Zachtronics", br{},
                "Webapp © OnlineMathLearning.com"
            }
        }
    }
}