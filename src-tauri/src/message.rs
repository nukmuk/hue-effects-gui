use bincode::Options;
use serde::Deserialize;
use serde::Serialize;

use crate::big_array::BigArray;

pub fn hello() {
    println!("Hello, world!");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    pub id: u8,
    pub color: (u16, u16, u16),
}

type EntertainmentID = [u8; 36];

#[derive(Serialize, Deserialize)]
pub struct MessageHead {
    pub protocol: [u8; 9],
    pub version: [u8; 2],
    pub sequence_number: u8,
    pub reserved: [u8; 2],
    pub color_mode: u8,
    pub reserved2: u8,
    #[serde(with = "BigArray")]
    pub entertainment_id: EntertainmentID,
}
pub struct Message {
    pub head: MessageHead,
    pub channels: Vec<Channel>,
}

impl Message {
    pub fn build(&self) -> Vec<u8> {
        let big_endian = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding();

        let mut head: Vec<u8> = bincode::serialize(&self.head).unwrap();
        // println!("channels: {:?}", &self.channels);
        self.channels.iter().for_each(|channel| {
            let mut channel_vec = big_endian.serialize(channel).unwrap();
            // println!("channel: {:?}", channel_vec);
            head.append(&mut channel_vec);
        });
        head
    }
}
