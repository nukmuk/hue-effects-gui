#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(dead_code)]
#![allow(unused_variables)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::env;

use serde::Serialize;

use std::sync::Mutex;

mod message;

mod big_array;

mod convert_color_to_rgb;

mod entertainment_config;

mod shader;

mod commands;
use commands::*;
mod commands_bridge;
use commands_bridge::*;
mod state_structs;
use state_structs::*;
mod util;
use util::*;

// use message::{Channel, HueColor, Message, MessageHead, Hello};

#[derive(Debug, Serialize, Clone)]
struct HueBridge {
    address: String,
    name: String,
}

#[derive(Default)]
struct AppKeys {
    application_key: Mutex<String>,
    psk: Mutex<String>,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(AppStateStruct {
            0: {
                Mutex::from(AppState {
                    rainbow: RainbowStats {
                        scale: 1.0,
                        speed: 1.0,
                        ..Default::default()
                    },
                    frequency: 50,
                    test_mode: true,
                    ..Default::default()
                })
            },
        })
        .manage(AppKeys {
            ..Default::default()
        })
        .manage(EffectStruct {
            effect: Mutex::from(Effect::Rainbow),
            solid_color: Mutex::from(colorsys::Rgb::new(0.0, 0.0, 0.0, None)),
        })
        .invoke_handler(tauri::generate_handler![
            fetch,
            start_stream,
            stop_stream,
            discover_bridges,
            stop_discover,
            edit_rainbow,
            set_test_mode,
            set_effect,
            set_solid_color
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
