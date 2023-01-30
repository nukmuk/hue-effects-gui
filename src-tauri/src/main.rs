#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(dead_code)]
#![allow(unused_variables)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use mdns_sd::{ServiceDaemon, ServiceEvent};

use std::env;
use std::fs;

use webrtc::dtls::Error;

use serde::Serialize;

use std::sync::Mutex;
use tauri::{State, Window};

mod message;

mod big_array;

mod convert_color_to_rgb;

mod entertainment_config;

mod shader;

mod commands;
use commands::*;
mod state_structs;
use state_structs::*;

// use message::{Channel, HueColor, Message, MessageHead, Hello};

#[derive(Debug, Serialize, Clone)]
struct HueBridge {
    address: String,
    name: String,
}

#[tauri::command]
async fn discover_bridges(state: State<'_, AppStateStruct>, window: Window) -> Result<String, ()> {
    if state.0.lock().unwrap().searching {
        return Ok("already searching".to_string());
    }
    state.0.lock().unwrap().searching = true;

    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Browse for a service type.
    let service_type = "_hue._tcp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");

    // Receive the browse events in sync or async. Here is
    // an example of using a thread. Users can call `receiver.recv_async().await`
    // if running in async environment.
    println!("part 1");
    while state.0.lock().unwrap().searching {
        println!("part 2");
        match receiver.recv().unwrap() {
            ServiceEvent::ServiceResolved(info) => {
                let bridge = HueBridge {
                    address: info.get_addresses().iter().next().unwrap().to_string(),
                    name: info.get_fullname().to_string(),
                };
                println!("Resolved a new service: {:?}", &bridge);
                window.emit("bridgeFound", &bridge).unwrap();
            }
            other_event => {
                println!("Received other event: {:?}", &other_event);
            }
        }
    }
    println!("end");
    Ok("started discover".to_string())
}

#[tauri::command]
async fn stop_discover(state: State<'_, AppStateStruct>) -> Result<(), ()> {
    println!("stopping discover");
    state.0.lock().unwrap().searching = false;
    Ok(())
}

#[tauri::command]
async fn stop_stream(state: tauri::State<'_, AppStateStruct>) -> Result<(), ()> {
    state.0.lock().unwrap().streaming = false;
    print!("STOPPED COMMAND: {:#?}", state);
    Ok(())
}

#[tauri::command]
async fn set_test_mode(enabled: bool, state: State<'_, AppStateStruct>) -> Result<(), ()> {
    println!("setting test mode to: {:?}", enabled);
    state.0.lock().unwrap().test_mode = enabled;
    Ok(())
}

fn getkey(hint: &[u8]) -> Result<Vec<u8>, Error> {
    let contents = fs::read_to_string("psk.txt").expect("Should have been able to read the file");

    println!("read file: {contents}");

    Ok(hex::decode(contents).unwrap())
}

#[derive(Default)]
struct AppKeys {
    username: String,
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
