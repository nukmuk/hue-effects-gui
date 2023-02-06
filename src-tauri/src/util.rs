use std::{hint, string};

use reqwest::header::{HeaderMap, HeaderValue};
use tokio::fs;

use crate::entertainment_config::EntertainmentConfig;

pub fn get_psk(hint: &[u8]) -> Result<Vec<u8>, webrtc::dtls::Error> {
    let contents = match std::fs::read_to_string("psk.txt") {
            Ok(fc) => fc,
            Err(e) => {
                println!("Error reading psk file: {:#?}", e);
                "".to_owned()
            },
        };
        
        println!("psk hint: {hint:#?}");
    println!("read psk file: {contents}");

    Ok(hex::decode(contents).unwrap())
}

pub async fn create_reqwest_client() -> reqwest::Client {
    let mut headers = HeaderMap::new();

    let contents = fs::read_to_string("hue-application-key.txt").await;
    let contents = match contents {
        Ok(c) => c,
        Err(e) => {
            println!("Error reading hue-application-key file: {:#?}", e);
            "".to_owned()
        },
    };

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

pub fn example_area() -> String {
    "{
        \"data\":
    [
        {
            \"id\":\"199e6eed-da27-488f-9184-7f0236913765\",
            \"type\":\"entertainment_configuration\",
            \"id_v1\":\"/groups/11\",
            \"name\":\"center\",
            \"status\":\"inactive\",
            \"configuration_type\":\"screen\",
            \"metadata\":{
               \"name\":\"center\"
            },
            \"stream_proxy\":{
               \"mode\":\"manual\",
               \"node\":{
                  \"rtype\":\"entertainment\",
                  \"rid\":\"2eec4f75-9d44-4b4b-830b-94b60faf5898\"
               }
            },
            \"channels\":[
               {
                  \"channel_id\":0,
                  \"position\":{
                     \"x\":1,
                     \"y\":-0.53306,
                     \"z\":-0.06718
                  },
                  \"members\":[
                     {
                        \"service\":{
                           \"rtype\":\"entertainment\",
                           \"rid\":\"5710c6f4-bd48-4662-b55f-ccede989f6f2\"
                        },
                        \"index\":0
                     }
                  ]
               },
               {
                  \"channel_id\":1,
                  \"position\":{
                     \"x\":0.00999,
                     \"y\":1,
                     \"z\":1
                  },
                  \"members\":[
                     {
                        \"service\":{
                           \"rtype\":\"entertainment\",
                           \"rid\":\"b1483692-a2dd-4b4e-a5c0-a02287c58eb1\"
                        },
                        \"index\":0
                     }
                  ]
               },
               {
                  \"channel_id\":2,
                  \"position\":{
                     \"x\":0.4,
                     \"y\":0.8,
                     \"z\":-0.4
                  },
                  \"members\":[
                     {
                        \"service\":{
                           \"rtype\":\"entertainment\",
                           \"rid\":\"2eec4f75-9d44-4b4b-830b-94b60faf5898\"
                        },
                        \"index\":0
                     }
                  ]
               },
               {
                  \"channel_id\":3,
                  \"position\":{
                     \"x\":0,
                     \"y\":0.8,
                     \"z\":-0.4
                  },
                  \"members\":[
                     {
                        \"service\":{
                           \"rtype\":\"entertainment\",
                           \"rid\":\"2eec4f75-9d44-4b4b-830b-94b60faf5898\"
                        },
                        \"index\":1
                     }
                  ]
               },
               {
                  \"channel_id\":4,
                  \"position\":{
                     \"x\":-0.4,
                     \"y\":0.8,
                     \"z\":-0.4
                  },
                  \"members\":[
                     {
                        \"service\":{
                           \"rtype\":\"entertainment\",
                           \"rid\":\"2eec4f75-9d44-4b4b-830b-94b60faf5898\"
                        },
                        \"index\":2
                     }
                  ]
               },
               {
                  \"channel_id\":5,
                  \"position\":{
                     \"x\":-1,
                     \"y\":0.12392,
                     \"z\":-0.23914
                  },
                  \"members\":[
                     {
                        \"service\":{
                           \"rtype\":\"entertainment\",
                           \"rid\":\"3130a81f-039f-49cd-bb26-f47c94f9d36d\"
                        },
                        \"index\":0
                     }
                  ]
               }
            ],
            \"locations\":{
               \"service_locations\":[
                  {
                     \"service\":{
                        \"rtype\":\"entertainment\",
                        \"rid\":\"5710c6f4-bd48-4662-b55f-ccede989f6f2\"
                     },
                     \"positions\":[
                        {
                           \"x\":1,
                           \"y\":-0.53306,
                           \"z\":-0.06718
                        }
                     ],
                     \"equalization_factor\":1,
                     \"position\":{
                        \"x\":1,
                        \"y\":-0.53306,
                        \"z\":-0.06718
                     }
                  },
                  {
                     \"service\":{
                        \"rtype\":\"entertainment\",
                        \"rid\":\"b1483692-a2dd-4b4e-a5c0-a02287c58eb1\"
                     },
                     \"positions\":[
                        {
                           \"x\":0.00999,
                           \"y\":1,
                           \"z\":1
                        }
                     ],
                     \"equalization_factor\":1,
                     \"position\":{
                        \"x\":0.00999,
                        \"y\":1,
                        \"z\":1
                     }
                  },
                  {
                     \"service\":{
                        \"rtype\":\"entertainment\",
                        \"rid\":\"2eec4f75-9d44-4b4b-830b-94b60faf5898\"
                     },
                     \"positions\":[
                        {
                           \"x\":0.68,
                           \"y\":1,
                           \"z\":-0.54
                        },
                        {
                           \"x\":-0.68,
                           \"y\":1,
                           \"z\":-0.54
                        }
                     ],
                     \"equalization_factor\":1,
                     \"position\":{
                        \"x\":0.68,
                        \"y\":1,
                        \"z\":-0.54
                     }
                  },
                  {
                     \"service\":{
                        \"rtype\":\"entertainment\",
                        \"rid\":\"3130a81f-039f-49cd-bb26-f47c94f9d36d\"
                     },
                     \"positions\":[
                        {
                           \"x\":-1,
                           \"y\":0.12392,
                           \"z\":-0.23914
                        }
                     ],
                     \"equalization_factor\":1,
                     \"position\":{
                        \"x\":-1,
                        \"y\":0.12392,
                        \"z\":-0.23914
                     }
                  }
               ]
            },
            \"light_services\":[
               {
                  \"rtype\":\"light\",
                  \"rid\":\"e8bb2746-1cdc-4b81-9d23-354526162114\"
               },
               {
                  \"rtype\":\"light\",
                  \"rid\":\"3670df9f-39a8-40a3-8a47-d568236471a8\"
               },
               {
                  \"rtype\":\"light\",
                  \"rid\":\"7583f0d8-1f9a-42b0-aab2-a124e1a3fd89\"
               },
               {
                  \"rtype\":\"light\",
                  \"rid\":\"fb8dc977-d6b9-4856-8102-9dd36a93c130\"
               }
            ]
         }
    ]
        }".to_owned()
}