use serde::{Deserialize, Serialize};

use crate::game::Skin;

#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub allow_undo: bool,
    pub unlock_difficulties: bool,
    pub unlock_difficulties_disabled: bool,
    pub skin: Skin,
}