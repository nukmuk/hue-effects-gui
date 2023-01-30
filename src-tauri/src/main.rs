#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(dead_code)]
#![allow(unused_variables)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use reqwest::header::HeaderValue;

use mdns_sd::{ServiceDaemon, ServiceEvent};
use serde::Deserialize;
use std::env;
use std::fs;
use std::sync::Arc;

use tokio::net::UdpSocket;

use webrtc::dtls::cipher_suite::CipherSuiteId;
use webrtc::dtls::Error;
use webrtc::dtls::{config::*, conn::DTLSConn};

use serde::Serialize;

use std::sync::Mutex;
use tauri::{State, Window};

use webrtc::util::Conn;

use colorsys::Hsl;

mod message;
use message::*;

mod big_array;

mod convert_color_to_rgb;
use convert_color_to_rgb::color_to_rgb_tuple;

mod entertainment_config;
use entertainment_config::EntertainmentConfig;

mod shader;
use shader::p2c;

// use message::{Channel, HueColor, Message, MessageHead, Hello};

#[derive(Debug)]
pub struct AppStateStruct(Mutex<AppState>);
#[derive(Debug, Default)]
struct AppState {
    streaming: bool,
    searching: bool,
    test_mode: bool,
    effect: Effect,
    frequency: u16,
    rainbow: RainbowStats,
}

#[derive(Debug, Default)]
struct RainbowStats {
    angle: f64,
    scale: f64,
    speed: f64,
    offset: f64,
}

#[tauri::command]
async fn edit_rainbow(
    angle: f64,
    scale: f64,
    speed: f64,
    state: State<'_, AppStateStruct>,
) -> Result<(), ()> {
    let mut state = state.0.lock().unwrap();
    let rainbow = &mut state.rainbow;

    if angle != -1.0 {
        rainbow.angle = angle;
    } else if scale != -1.0 {
        rainbow.scale = scale;
    } else if speed != -1.0 {
        rainbow.speed = speed;
    }

    Ok(())
}

#[tauri::command]
async fn fetch(url: &str) -> Result<String, ()> {
    let mut headers = reqwest::header::HeaderMap::new();

    let contents = fs::read_to_string("hue-application-key.txt")
        .expect("Should have been able to read the file");

    headers.append(
        "hue-application-key",
        HeaderValue::from_str(&contents).unwrap(),
    );

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap_or_default();
    let body = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or_default();

    Ok(body)
}

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
async fn start_stream(
    url: &str,
    state: tauri::State<'_, AppStateStruct>,
    window: Window,
) -> Result<(), ()> {
    state.0.lock().unwrap().streaming = true;

    let mut headers = reqwest::header::HeaderMap::new();

    let contents = fs::read_to_string("hue-application-key.txt")
        .expect("Should have been able to read the file");

    headers.append(
        "hue-application-key",
        HeaderValue::from_str(&contents).unwrap(),
    );

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap_or_default();
    let body = client
        .put(url)
        .headers(headers)
        .body("{\"action\":\"start\"}")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or_default();

    let bridge_ip = "192.168.1.21:2100";

    let color_hsl = Hsl::new(0.0, 100.0, 50.0, None);
    let color_tuple = color_to_rgb_tuple(&color_hsl);
    // println!("{:?}", color_tuple);

    let fetched_channels = fetch("https://192.168.1.21/clip/v2/resource/entertainment_configuration/199e6eed-da27-488f-9184-7f0236913765").await.unwrap();

    let ent_config: EntertainmentConfig = serde_json::from_str(&fetched_channels).unwrap();
    let data = &ent_config.data.first().unwrap().channels;

    let conn = Arc::new(UdpSocket::bind("0.0.0.0:0").await.unwrap());
    conn.connect(bridge_ip).await.unwrap();
    println!("connecting {}..", bridge_ip);

    let config = Config {
        psk: Some(Arc::new(getkey)),
        psk_identity_hint: Some("webrtc-rs DTLS Server".as_bytes().to_vec()),
        cipher_suites: vec![CipherSuiteId::Tls_Psk_With_Aes_128_Gcm_Sha256],
        extended_master_secret: ExtendedMasterSecretType::Require,
        ..Default::default()
    };
    let dtls_conn: Arc<dyn Conn + Send + Sync> =
        Arc::new(DTLSConn::new(conn, config, true, None).await.unwrap());

    println!("Connected;");

    let mut time_step = 0.0;

    while state.0.lock().unwrap().streaming {
        let frequency = state.0.lock().unwrap().frequency;
        let increment = state.0.lock().unwrap().rainbow.speed / 10.0;
        println!("increment: {}", increment);
        let mut msg_channels = Vec::new();
        let channel_count = data.len() as u8;
        data.iter().for_each(|channel| {
            let color = p2c(
                channel.position.x,
                channel.position.y,
                channel.position.z,
                time_step,
                channel.channel_id,
                channel_count,
                &state,
            );

            let result = Channel {
                id: channel.channel_id,
                color,
            };
            msg_channels.push(result);
        });

        let test_msg = Message {
            head: MessageHead {
                protocol: [b'H', b'u', b'e', b'S', b't', b'r', b'e', b'a', b'm'],
                version: [0x02, 0x00],
                sequence_number: 0x07,
                reserved: [0x00, 0x00],
                color_mode: 0x00,
                reserved2: 0x00,
                entertainment_id: [
                    0x31, 0x39, 0x39, 0x65, 0x36, 0x65, 0x65, 0x64, 0x2D, 0x64, 0x61, 0x32, 0x37,
                    0x2D, 0x34, 0x38, 0x38, 0x66, 0x2D, 0x39, 0x31, 0x38, 0x34, 0x2D, 0x37, 0x66,
                    0x30, 0x32, 0x33, 0x36, 0x39, 0x31, 0x33, 0x37, 0x36, 0x35,
                ],
            },
            channels: msg_channels,
        };

        // println!("channels: {:#?}", data);
        let msg_built = test_msg.build();

        window.emit("lightUpdate", &test_msg.channels).unwrap();

        if !state.0.lock().unwrap().test_mode {
            dtls_conn.send(&*msg_built).await.expect("failed to send"); // &* syntax converts Vec to array
        }
        time_step += increment;
        // println!("{:?}", msg_built);
        // sleep(time::Duration::from_millis(1000 / 50));
        tokio::time::sleep(tokio::time::Duration::from_millis(
            (1000 / &frequency).into(),
        ))
        .await;
    }

    dtls_conn.close().await.unwrap();
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

#[derive(Debug, Deserialize, Default, PartialEq, Clone, Copy)]
enum Effect {
    #[default]
    Rainbow,
    Flash,
    Solid,
    None,
}

#[tauri::command]
async fn set_effect(effect: Effect, state: State<'_, AppStateStruct>) -> Result<(), ()> {
    println!("setting effect to: {:?}", effect);
    state.0.lock().unwrap().effect = effect;
    Ok(())
}

fn getkey(hint: &[u8]) -> Result<Vec<u8>, Error> {
    let contents = fs::read_to_string("psk.txt").expect("Should have been able to read the file");

    println!("read file: {contents}");

    Ok(hex::decode(contents).unwrap())
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
        .invoke_handler(tauri::generate_handler![
            fetch,
            start_stream,
            stop_stream,
            discover_bridges,
            stop_discover,
            edit_rainbow,
            set_test_mode,
            set_effect
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
