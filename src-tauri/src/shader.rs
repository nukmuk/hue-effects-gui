use tauri::State;

use crate::{
    convert_color_to_rgb::{hsl_to_tuple, rgb_to_tuple},
    state_structs::{Effect, EffectStruct},
    AppStateStruct,
};

pub fn calculate_light_color(
    x: f64,
    y: f64,
    z: f64,
    time_step: f64,
    channel_id: u8,
    state: &State<'_, AppStateStruct>,
    effect_state: &State<'_, EffectStruct>,
    flash_n: u8,
) -> (u16, u16, u16) {
    match *effect_state.effect.lock().unwrap() {
        Effect::Rainbow => {
            let rainbow = &state.0.lock().unwrap().rainbow;
            let hue = ((x) * rainbow.scale * 100.0 + time_step).rem_euclid(360.0);

            let color = colorsys::Hsl::from((hue, 100.0, 50.0));

            let result = hsl_to_tuple(&color);
            result
        }
        Effect::Flash => {
            println!("rng: {}", flash_n);
            println!("channel_id: {}", channel_id);
            let color = &*effect_state.solid_color.lock().unwrap();
            if channel_id == flash_n {
                rgb_to_tuple(color)
            } else {
                (0, 0, 0)
            }
        }
        Effect::Solid => rgb_to_tuple(&*effect_state.solid_color.lock().unwrap()),
        _ => (0, 0, 0),
    }
}
