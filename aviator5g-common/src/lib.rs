/*
 * AVIATOR5G SYSTEM
 * Copyright (c) 2021 SilentByte <https://silentbyte.com/>
 */

use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

pub type Id = Uuid;

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
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ControlMessage {
    Identification(IdentificationMessageData),
    Control(ControlMessageData),
}

pub fn parse_control_message(message: &str) -> Result<ControlMessage, String> {
    serde_json::from_str(&message).map_err(|e| e.to_string())
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
