/*
 * AVIATOR5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

pub type Id = Uuid;
pub type DateTime = chrono::DateTime<chrono::Utc>;

pub fn id_from_str(id: &str) -> Id {
    Uuid::from_str(id).expect("Invalid UUID string")
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClientType {
    Pilot,
    Vehicle,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IdentificationMessageData {
    pub id: Id,
    pub group_id: Id,
    pub client_type: ClientType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ControlMessageData {
    pub axes: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LatencyRequestMessageData {
    pub initiator_id: Id,
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LatencyResponseMessageData {
    pub initiator_id: Id,
    pub responder_id: Id,
    pub timestamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ControlMessage {
    Identification(IdentificationMessageData),
    Control(ControlMessageData),
    LatencyRequest(LatencyRequestMessageData),
    LatencyResponse(LatencyResponseMessageData),
}

pub fn parse_control_message(message: &str) -> Result<ControlMessage, String> {
    serde_json::from_str(&message).map_err(|e| e.to_string())
}

pub fn build_control_message(control_message: &ControlMessage) -> String {
    serde_json::to_string(control_message).expect("Could not build control message")
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
