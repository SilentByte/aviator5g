/*
 * AVIATOR5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

use std::sync::{
    Arc,
    Mutex,
};
use std::time::Duration;

use argh::FromArgs;
use aviator5g_common::{
    ClientType,
    ControlMessage,
    ControlMessageData,
};
use futures_util::{
    SinkExt,
    StreamExt,
    TryStreamExt,
};

/// Aviator5G Vehicle.
#[derive(Debug, Clone, FromArgs)]
struct Args {
    /// the server's endpoint to which this vehicle should attempt to connect.
    #[argh(option)]
    url: String,
}

fn lerp(start: f64, end: f64, amount: f64) -> f64 {
    (1.0 - amount) * start + amount * end
}

const DEFAULT_SERVO_PERIOD: Duration = Duration::from_micros(20000);
const DEFAULT_SERVO_PULSE_MIN: Duration = Duration::from_micros(771);
const DEFAULT_PULSE_NEUTRAL: Duration = Duration::from_micros(1500);
const DEFAULT_PULSE_MAX: Duration = Duration::from_micros(2193);

#[derive(Debug)]
struct Servo {
    pulse_min: Duration,
    pulse_neutral: Duration,
    pulse_max: Duration,
    pwm: rppal::pwm::Pwm,
}

impl Servo {
    fn new(
        period: Duration,
        pulse_min: Duration,
        pulse_neutral: Duration,
        pulse_max: Duration,
        channel: rppal::pwm::Channel,
    ) -> anyhow::Result<Self> {
        let servo = Self {
            pulse_min,
            pulse_neutral,
            pulse_max,
            pwm: rppal::pwm::Pwm::with_period(
                channel,
                period,
                pulse_neutral,
                rppal::pwm::Polarity::Normal,
                true,
            )?,
        };

        Ok(servo)
    }

    fn rotate(&mut self, amount: f64) -> anyhow::Result<()> {
        let amount = amount.clamp(-1.0, 1.0);
        let pulse_us = if amount < 0.0 {
            lerp(
                self.pulse_neutral.as_micros() as f64,
                self.pulse_min.as_micros() as f64,
                -amount as f64,
            ) as u64
        } else if amount > 0.0 {
            lerp(
                self.pulse_neutral.as_micros() as f64,
                self.pulse_max.as_micros() as f64,
                amount as f64,
            ) as u64
        } else {
            self.pulse_neutral.as_micros() as u64
        };

        self.pwm.set_pulse_width(Duration::from_micros(pulse_us))?;

        Ok(())
    }
}

#[derive(Debug)]
struct VehicleController {
    ailerons_axis: f64,
    ailerons_servo: Servo,
    elevator_axis: f64,
    elevator_servo: Servo,
}

impl VehicleController {
    fn new() -> anyhow::Result<Self> {
        let controller = Self {
            ailerons_axis: 0.0,
            ailerons_servo: Servo::new(
                DEFAULT_SERVO_PERIOD,
                DEFAULT_SERVO_PULSE_MIN,
                DEFAULT_PULSE_NEUTRAL,
                DEFAULT_PULSE_MAX,
                rppal::pwm::Channel::Pwm0,
            )?,
            elevator_axis: 0.0,
            elevator_servo: Servo::new(
                DEFAULT_SERVO_PERIOD,
                DEFAULT_SERVO_PULSE_MIN,
                DEFAULT_PULSE_NEUTRAL,
                DEFAULT_PULSE_MAX,
                rppal::pwm::Channel::Pwm1,
            )?,
        };

        Ok(controller)
    }

    fn update_from_control_message_data(&mut self, data: ControlMessageData) {
        if data.axes.len() != 2 {
            panic!("Expected data for exactly 2 axes");
        }

        self.ailerons_axis = data.axes[0];
        self.elevator_axis = data.axes[1];

        self.ailerons_servo.rotate(self.ailerons_axis).unwrap();
        self.elevator_servo.rotate(self.elevator_axis).unwrap();
    }
}

const VEHICLE_GROUP_ID: &str = "14ed4af8-5256-4e74-a5d6-545dfc0b004c";
const VEHICLE_ID: &str = "e72029c7-ce0f-45c7-bc3a-3e01e5c53944";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args: Args = argh::from_env();
    let url = url::Url::parse(&args.url)?;

    log::info!("Connecting to server at {}", url);
    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
    let (mut outgoing, incoming) = ws_stream.split();

    outgoing
        .send(tungstenite::Message::Text(
            aviator5g_common::build_control_message(
                &aviator5g_common::ControlMessage::Identification(
                    aviator5g_common::IdentificationMessageData {
                        group_id: aviator5g_common::id_from_str(VEHICLE_GROUP_ID),
                        id: aviator5g_common::id_from_str(VEHICLE_ID),
                        client_type: ClientType::Vehicle,
                    },
                ),
            ),
        ))
        .await
        .expect("Failed to send identification payload");

    let vehicle_state = Arc::new(Mutex::new(VehicleController::new()?));
    let outgoing = Arc::new(Mutex::new(outgoing));

    incoming
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
                        ControlMessage::LatencyRequest(data) => {
                            outgoing
                                .lock()
                                .unwrap()
                                .send(tungstenite::Message::Text(
                                    aviator5g_common::build_control_message(
                                        &ControlMessage::LatencyResponse(
                                            aviator5g_common::LatencyResponseMessageData {
                                                initiator_id: data.initiator_id,
                                                responder_id: aviator5g_common::id_from_str(
                                                    VEHICLE_ID,
                                                ),
                                                timestamp: data.timestamp,
                                            },
                                        ),
                                    ),
                                ))
                                .await
                                .unwrap();
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
