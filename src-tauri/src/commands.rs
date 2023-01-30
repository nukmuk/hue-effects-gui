use std::sync::Arc;

use colorsys::{Hsl, Rgb};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use tauri::{State, Window};

use tokio::{fs, net::UdpSocket};
use webrtc::{
    dtls::{
        cipher_suite::CipherSuiteId,
        config::{Config, ExtendedMasterSecretType},
        conn::DTLSConn,
    },
    util::Conn,
};

use crate::{
    convert_color_to_rgb::hsl_to_tuple,
    entertainment_config::EntertainmentConfig,
    get_psk,
    message::{Channel, Message, MessageHead},
    shader::p2c,
    state_structs::{Effect, EffectStruct},
    AppKeys, AppStateStruct,
};

async fn create_reqwest_client() -> Client {
    let mut headers = HeaderMap::new();

    let contents = fs::read_to_string("hue-application-key.txt")
        .await
        .expect("Should have been able to read the file");

    headers.append(
        "hue-application-key",
        HeaderValue::from_str(&contents).unwrap(),
    );
    reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .default_headers(headers)
        .build()
        .unwrap_or_default()
}

#[tauri::command]
pub async fn set_effect(effect: Effect, state: State<'_, EffectStruct>) -> Result<(), ()> {
    println!("setting effect to: {:?}", effect);
    *state.effect.lock().unwrap() = effect;
    Ok(())
}

#[tauri::command]
pub async fn set_solid_color(color: String, state: State<'_, EffectStruct>) -> Result<(), ()> {
    let color = Rgb::from_hex_str(&color).unwrap();
    println!("setting color to: {:?}", color);
    *state.solid_color.lock().unwrap() = color;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct HueOptions {
    frequency: Option<u8>,
}

#[tauri::command]
pub fn edit_options(options: HueOptions, state: State<'_, AppStateStruct>) -> Result<(), ()> {
    if options.frequency.is_some() {
        println!("setting frequency to: {:?}", options.frequency);
        state.0.lock().unwrap().frequency = options.frequency.unwrap();
    }
    Ok(())
}

#[tauri::command]
pub async fn edit_rainbow(
    angle: Option<f64>,
    scale: Option<f64>,
    speed: Option<f64>,
    state: State<'_, AppStateStruct>,
) -> Result<(), ()> {
    let mut state = state.0.lock().unwrap();
    let rainbow = &mut state.rainbow;

    if angle.is_some() {
        println!("setting angle to: {:?}", angle);
        rainbow.angle = angle.unwrap();
    } else if scale.is_some() {
        println!("setting scale to: {:?}", scale);
        rainbow.scale = scale.unwrap();
    } else if speed.is_some() {
        println!("setting speed to: {:?}", speed);
        rainbow.speed = speed.unwrap();
    }

    Ok(())
}

#[tauri::command]
pub async fn fetch(url: &str) -> Result<String, ()> {
    let client = create_reqwest_client().await;

    let body = client
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or_default();

    Ok(body)
}

#[tauri::command]
pub async fn start_stream(
    url: &str,
    state: tauri::State<'_, AppStateStruct>,
    effect_state: tauri::State<'_, EffectStruct>,
    keys_state: tauri::State<'_, AppKeys>,
    window: Window,
) -> Result<(), ()> {
    state.0.lock().unwrap().streaming = true;

    let client = create_reqwest_client().await;

    let body = client
        .put(url)
        .body("{\"action\":\"start\"}")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or_default();

    let bridge_ip = "192.168.1.21:2100";

    let color_hsl = Hsl::new(0.0, 100.0, 50.0, None);
    let color_tuple = hsl_to_tuple(&color_hsl);
    // println!("{:?}", color_tuple);

    let fetched_channels = fetch("https://192.168.1.21/clip/v2/resource/entertainment_configuration/199e6eed-da27-488f-9184-7f0236913765").await.unwrap();

    let ent_config: EntertainmentConfig = serde_json::from_str(&fetched_channels).unwrap();
    let data = &ent_config.data.first().unwrap().channels;

    let conn = Arc::new(UdpSocket::bind("0.0.0.0:0").await.unwrap());
    conn.connect(bridge_ip).await.unwrap();
    println!("connecting {}..", bridge_ip);

    let config = Config {
        psk: Some(Arc::new(get_psk)),
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
        let effect = *effect_state.effect.lock().unwrap();
        data.iter().for_each(|channel| {
            let color = p2c(
                channel.position.x,
                channel.position.y,
                channel.position.z,
                time_step,
                channel.channel_id,
                channel_count,
                &state,
                &effect_state,
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
        let sleep_time = 1000 / frequency as u64;
        tokio::time::sleep(tokio::time::Duration::from_millis(sleep_time)).await;
    }

    dtls_conn.close().await.unwrap();
    Ok(())
}

#[tauri::command]
pub async fn stop_discover(state: State<'_, AppStateStruct>) -> Result<(), ()> {
    println!("stopping discover");
    state.0.lock().unwrap().searching = false;
    Ok(())
}

#[tauri::command]
pub async fn stop_stream(state: tauri::State<'_, AppStateStruct>) -> Result<(), ()> {
    state.0.lock().unwrap().streaming = false;
    print!("stopped stream: {:#?}", state);
    Ok(())
}

#[tauri::command]
pub async fn set_test_mode(enabled: bool, state: State<'_, AppStateStruct>) -> Result<(), ()> {
    println!("setting test mode to: {:?}", enabled);
    state.0.lock().unwrap().test_mode = enabled;
    Ok(())
}
