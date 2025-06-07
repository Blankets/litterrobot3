use serde::{Deserialize, Serialize};

pub enum Command {
    Cycle,
    PowerOff,
    PowerOn,
    LockOff,
    LockOn,
    NightLightOff,
    NightLightOn,
    SleepModeOff,
    SleepModeOn,
}

impl Command {
    const PREFIX: &str = "<";

    fn serialize(&self) -> String {
        let command = match self {
            Self::Cycle => "C",
            Self::PowerOff => "P0",
            Self::PowerOn => "P1",
            Self::LockOff => "L0",
            Self::LockOn => "L1",
            Self::NightLightOff => "N0",
            Self::NightLightOn => "N1",
            Self::SleepModeOff => "S0",
            Self::SleepModeOn => "S1",
        };

        format!("{}{command}", Self::PREFIX)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRequest {
    pub litter_robot_id: String,
    pub command: String,
}

impl CommandRequest {
    pub fn new(robot_id: &str, command: Command) -> Self {
        Self {
            litter_robot_id: robot_id.to_string(),
            command: command.serialize(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResponse {
    pub command: CommandOutput,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandOutput {
    pub command: String,
    pub litter_robot_id: String,
    pub litter_robot_serial: String,
    pub timestamp: String,
    pub ttl: u64,
}
