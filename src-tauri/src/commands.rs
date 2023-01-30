use std::sync::Arc;

use colorsys::Hsl;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use tauri::Window;
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
    convert_color_to_rgb::color_to_rgb_tuple,
    entertainment_config::EntertainmentConfig,
    getkey,
    message::{Channel, Message, MessageHead},
    shader::p2c,
    AppStateStruct,
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
