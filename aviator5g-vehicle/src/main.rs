/*
 * AVIATOR 5G SYSTEM
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
const DEFAULT_SERVO_PULSE_MIN: Duration = Duration::from_micros(1000);
const DEFAULT_SERVO_PULSE_NEUTRAL: Duration = Duration::from_micros(1500);
const DEFAULT_SERVO_PULSE_MAX: Duration = Duration::from_micros(2000);

#[derive(Debug)]
enum ServoPin {
    Pwm0,  // Pin: GPIO 18 / Physical 12.
    Pwm1,  // Pin: GPIO 19 / Physical 35.
    Soft0, // Pin: GPIO 23 / Physical 16.
    Soft1, // Pin: GPIO 24 / Physical 18.
}

#[derive(Debug)]
enum ServoConnection {
    Hard(rppal::pwm::Pwm),
    Soft(rppal::gpio::OutputPin),
}

#[derive(Debug)]
struct Servo {
    period: Duration,
    pulse_min: Duration,
    pulse_neutral: Duration,
    pulse_max: Duration,
    connection: ServoConnection,
}

impl Servo {
    fn new(
        period: Duration,
        pulse_min: Duration,
        pulse_neutral: Duration,
        pulse_max: Duration,
        pin: ServoPin,
    ) -> anyhow::Result<Self> {
        let connection = match pin {
            ServoPin::Pwm0 => ServoConnection::Hard(rppal::pwm::Pwm::with_period(
                rppal::pwm::Channel::Pwm0,
                period,
                pulse_neutral,
                rppal::pwm::Polarity::Normal,
                true,
            )?),
            ServoPin::Pwm1 => ServoConnection::Hard(rppal::pwm::Pwm::with_period(
                rppal::pwm::Channel::Pwm1,
                period,
                pulse_neutral,
                rppal::pwm::Polarity::Normal,
                true,
            )?),
            ServoPin::Soft0 => {
                ServoConnection::Soft(rppal::gpio::Gpio::new()?.get(23)?.into_output())
            }
            ServoPin::Soft1 => {
                ServoConnection::Soft(rppal::gpio::Gpio::new()?.get(24)?.into_output())
            }
        };

        let mut servo = Self {
            period,
            pulse_min,
            pulse_neutral,
            pulse_max,
            connection,
        };

        servo.rotate(0.0)?;

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

        match &mut self.connection {
            ServoConnection::Hard(c) => c.set_pulse_width(Duration::from_micros(pulse_us))?,
            ServoConnection::Soft(c) => c.set_pwm(self.period, Duration::from_micros(pulse_us))?,
        }

        Ok(())
    }

    fn disable(&mut self) -> anyhow::Result<()> {
        match &mut self.connection {
            ServoConnection::Hard(c) => c.disable()?,
            ServoConnection::Soft(c) => c.set_low(),
        }

        Ok(())
    }
}

#[derive(Debug)]
struct VehicleController {
    ailerons_axis: f64,
    ailerons_servo: Servo,
    elevator_axis: f64,
    elevator_servo: Servo,
    rudder_axis: f64,
    rudder_servo: Servo,
    throttle_axis: f64,
    throttle_servo: Servo,
}

impl VehicleController {
    fn new() -> anyhow::Result<Self> {
        let controller = Self {
            ailerons_axis: 0.0,
            ailerons_servo: Servo::new(
                DEFAULT_SERVO_PERIOD,
                DEFAULT_SERVO_PULSE_MIN,
                DEFAULT_SERVO_PULSE_NEUTRAL,
                DEFAULT_SERVO_PULSE_MAX,
                ServoPin::Pwm0,
            )?,
            elevator_axis: 0.0,
            elevator_servo: Servo::new(
                DEFAULT_SERVO_PERIOD,
                DEFAULT_SERVO_PULSE_MIN,
                DEFAULT_SERVO_PULSE_NEUTRAL,
                DEFAULT_SERVO_PULSE_MAX,
                ServoPin::Pwm1,
            )?,
            rudder_axis: 0.0,
            rudder_servo: Servo::new(
                DEFAULT_SERVO_PERIOD,
                DEFAULT_SERVO_PULSE_MIN,
                DEFAULT_SERVO_PULSE_NEUTRAL,
                DEFAULT_SERVO_PULSE_MAX,
                ServoPin::Soft0,
            )?,
            throttle_axis: 0.0,
            throttle_servo: Servo::new(
                DEFAULT_SERVO_PERIOD,
                DEFAULT_SERVO_PULSE_NEUTRAL, // Prevent throttle from going negative.
                DEFAULT_SERVO_PULSE_NEUTRAL,
                DEFAULT_SERVO_PULSE_MAX,
                ServoPin::Soft1,
            )?,
        };

        Ok(controller)
    }

    fn update_from_control_message_data(&mut self, data: ControlMessageData) {
        if data.axes.len() != 4 {
            log::error!("Expected data for exactly 4 axes");
            return;
        }

        self.ailerons_axis = data.axes[0];
        self.elevator_axis = data.axes[1];
        self.rudder_axis = data.axes[2];
        self.throttle_axis = data.axes[3];

        self.ailerons_servo.rotate(self.ailerons_axis).unwrap();
        self.elevator_servo.rotate(self.elevator_axis).unwrap();
        self.rudder_servo.rotate(self.rudder_axis).unwrap();
        self.throttle_servo.rotate(self.throttle_axis).unwrap();
    }

    fn set_all_neutral(&mut self) {
        self.ailerons_axis = 0.0;
        self.elevator_axis = 0.0;
        self.rudder_axis = 0.0;
        self.throttle_axis = 0.0;

        self.ailerons_servo.rotate(0.0).unwrap();
        self.elevator_servo.rotate(0.0).unwrap();
        self.rudder_servo.rotate(0.0).unwrap();
        self.throttle_servo.rotate(0.0).unwrap();
    }

    fn disable_all(&mut self) {
        self.set_all_neutral();

        self.ailerons_servo.disable().unwrap();
        self.elevator_servo.disable().unwrap();
        self.rudder_servo.disable().unwrap();
        self.throttle_servo.disable().unwrap();
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

    let vehicle_controller = Arc::new(Mutex::new(VehicleController::new()?));
    let outgoing = Arc::new(Mutex::new(outgoing));

    simple_signal::set_handler(
        &[simple_signal::Signal::Int, simple_signal::Signal::Term],
        {
            let vehicle_controller = vehicle_controller.clone();
            move |_| {
                vehicle_controller.lock().unwrap().disable_all();
            }
        },
    );

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
                            let vehicle_controller = vehicle_controller.clone();
                            vehicle_controller
                                .lock()
                                .unwrap()
                                .update_from_control_message_data(data);

                            log::info!("Vehicle state updated: {:?}", vehicle_controller);
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

    vehicle_controller.lock().unwrap().disable_all();

    Ok(())
}
