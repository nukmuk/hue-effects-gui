use tauri::State;

use crate::{convert_color_to_rgb::color_to_rgb_tuple, AppStateStruct};

pub fn p2c(
    x: f64,
    y: f64,
    z: f64,
    time_step: f64,
    channel_id: u8,
    channel_count: u8,
    state: &State<'_, AppStateStruct>,
) -> (u16, u16, u16) {
    // let effect = &state.0.lock().unwrap().effect;
    // Mutex
    // let new_effect = effect.to_owned();
    // let mut output = (0, 0, 0);
    // // let output = match &*effect {
    // //     Rainbow => {
    // if new_effect == Rainbow {
    let rainbow = &state.0.lock().unwrap().rainbow;
    let hue = ((x) * rainbow.scale * 100.0 + time_step).rem_euclid(360.0);

    let color = colorsys::Hsl::from((hue, 100.0, 50.0));

    let result = color_to_rgb_tuple(&color);
    // }
    // }
    //     Flash => (0, 0, 0),
    //     Solid => (65535, 0, 0),
    //     None => (0, 0, 0),
    // };
    result
    // (0, 0, 0)
}
