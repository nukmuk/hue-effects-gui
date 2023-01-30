use std::sync::Mutex;

use serde::Deserialize;

#[derive(Debug)]
pub struct AppStateStruct(pub Mutex<AppState>);
#[derive(Debug, Default)]
pub struct AppState {
    pub streaming: bool,
    pub searching: bool,
    pub test_mode: bool,
    pub effect: Effect,
    pub frequency: u16,
    pub rainbow: RainbowStats,
}

#[derive(Debug, Deserialize, Default, PartialEq, Clone, Copy)]
pub enum Effect {
    #[default]
    Rainbow,
    Flash,
    Solid,
    None,
}

#[derive(Debug, Default)]
pub struct RainbowStats {
    pub angle: f64,
    pub scale: f64,
    pub speed: f64,
    pub offset: f64,
}
