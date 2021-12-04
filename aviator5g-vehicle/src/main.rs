/*
 * AVIATOR5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

use std::sync::{
    Arc,
    Mutex,
};

use argh::FromArgs;
use aviator5g_common::{
    ClientType,
    ControlMessage,
    ControlMessageData,
};
use futures_util::{
    SinkExt,
    TryStreamExt,
};

/// Aviator5G Vehicle.
#[derive(Debug, Clone, FromArgs)]
struct Args {
    /// the server's endpoint to which this vehicle should attempt to connect.
    #[argh(option)]
    url: String,
}

#[derive(Debug)]
struct VehicleState {
    ailerons: f64,
    elevator: f64,
}

impl VehicleState {
    fn new() -> Self {
        Self {
            ailerons: 0.0,
            elevator: 0.0,
        }
    }

    fn update_from_control_message_data(&mut self, data: ControlMessageData) {
        if data.axes.len() != 2 {
            panic!("Expected data for exactly 2 axes");
        }

        self.ailerons = data.axes[0];
        self.elevator = data.axes[1];
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args: Args = argh::from_env();
    let url = url::Url::parse(&args.url)?;

    log::info!("Connecting to server at {}", url);
    let (mut ws_stream, _) = tokio_tungstenite::connect_async(url).await?;

    ws_stream
        .send(tungstenite::Message::Text(
            aviator5g_common::build_control_message(
                &aviator5g_common::ControlMessage::Identification(
                    aviator5g_common::IdentificationMessageData {
                        group_id: aviator5g_common::id_from_str(
                            "14ed4af8-5256-4e74-a5d6-545dfc0b004c",
                        ),
                        id: aviator5g_common::id_from_str("e72029c7-ce0f-45c7-bc3a-3e01e5c53944"),
                        client_type: ClientType::Vehicle,
                    },
                ),
            ),
        ))
        .await
        .expect("Failed to send identification payload");

    let vehicle_state = Arc::new(Mutex::new(VehicleState::new()));
    ws_stream
        .try_for_each(|message| async {
            match message {
                tungstenite::Message::Text(text) => {
                    log::debug!("Received Text Message");

                    let control_message = aviator5g_common::parse_control_message(&text)
                        .expect("Control message is malformed");

                    log::debug!("Recieved Control Message: {:?}", control_message);
                    match control_message {
                        ControlMessage::Control(data) => {
                            let vehicle_state = vehicle_state.clone();
                            vehicle_state
                                .lock()
                                .unwrap()
                                .update_from_control_message_data(data);
                            log::info!("Vehicle state updated: {:?}", vehicle_state);
                        }
                        _ => {}
                    }
                }

                tungstenite::Message::Binary(_) => {
                    log::debug!("Received Binary Message");
                }
                tungstenite::Message::Ping(_) => {
                    log::debug!("Received Ping Message");
                }
                tungstenite::Message::Pong(_) => {
                    log::debug!("Received Pong Message");
                }
                tungstenite::Message::Close(_) => {
                    log::debug!("Received Close Message");
                }
            }

            Ok(())
        })
        .await
        .unwrap();

    Ok(())
}
