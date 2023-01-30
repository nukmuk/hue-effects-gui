use tauri::State;

use crate::{
    convert_color_to_rgb::{hsl_to_tuple, rgb_to_tuple},
    state_structs::{Effect, EffectStruct},
    AppStateStruct,
};

pub fn p2c(
    x: f64,
    y: f64,
    z: f64,
    time_step: f64,
    channel_id: u8,
    channel_count: u8,
    state: &State<'_, AppStateStruct>,
    effect_state: &State<'_, EffectStruct>,
) -> (u16, u16, u16) {
    match *effect_state.effect.lock().unwrap() {
        Effect::Rainbow => {
            let rainbow = &state.0.lock().unwrap().rainbow;
            let hue = ((x) * rainbow.scale * 100.0 + time_step).rem_euclid(360.0);

            let color = colorsys::Hsl::from((hue, 100.0, 50.0));

            let result = hsl_to_tuple(&color);
            result
        }
        Effect::Flash => (0, 0, 65535),
        Effect::Solid => rgb_to_tuple(&*effect_state.solid_color.lock().unwrap()),
        _ => (0, 0, 0),
    }
}
