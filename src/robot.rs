use serde::Deserialize;
use serde::de;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Robot {
    pub litter_robot_id: String,
    pub litter_robot_serial: String,
    pub litter_robot_nickname: String,
    pub device_type: String,
    #[serde(deserialize_with = "deserialize_u64")]
    pub cycle_count: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub total_cycle_count: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub cycle_capacity: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub new_cycle_capacity: u64,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(rename = "isDFITriggered")]
    pub is_dfi_triggered: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_df1_triggered: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_df2_triggered: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_dfs_triggered: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_manual_reset: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub saved_is_manual_reset: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(rename = "previousDFITriggered")]
    pub previous_dfi_triggered: bool,
    #[serde(deserialize_with = "deserialize_u64")]
    #[serde(rename = "DFICycleCount")]
    pub dfi_cycle_count: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub saved_cycle_count: u64,
    pub clean_cycle_wait_time_minutes: String,
    pub cycles_after_drawer_full: u64,
    #[serde(deserialize_with = "deserialize_bool")]
    pub night_light_active: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub panel_lock_active: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub sleep_mode_active: bool,
    pub sleep_mode_time: u64,
    pub power_status: String,
    pub unit_status: UnitStatus,
    #[serde(deserialize_with = "deserialize_u64")]
    pub sleep_mode_end_time: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub sleep_mode_start_time: u64,
    pub last_seen: String,
    pub setup_date: String,
    pub is_onboarded: bool,
    pub did_notify_offline: bool,
    pub auto_offline_disabled: bool,
    pub scoops_saved_count: u64,
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.litter_robot_nickname, self.unit_status)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum UnitStatus {
    RDY,
    CCP,
    CCC,
    CSF,
    DF1,
    DF2,
    CST,
    CSI,
    BR,
    P,
    OFF,
    SDF,
    DFS,
}

impl std::fmt::Display for UnitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::RDY => "Unit is ready",
            Self::CCP => "Clean cycle in progress",
            Self::CCC => "Clean cycle complete",
            Self::CSF => "Cat sensor fault",
            Self::DF1 => "Drawer full - 2 cycles remaining",
            Self::DF2 => "Drawer full - 1 cycle remaining",
            Self::CST => "Cat sensor timing",
            Self::CSI => "Cat sensor interrupted",
            Self::BR => "Bonnet removed",
            Self::P => "Unit is paused",
            Self::OFF => "Unit is powered off",
            Self::SDF => "Drawer full on startup and will not cycle",
            Self::DFS => "Drawer full and will not cycle",
        };

        write!(f, "{}", s)
    }
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        Value::Bool(b) => b,
        Value::String(s) => match s.as_str() {
            "0" | "false" => false,
            "1" | "true" => true,
            _ => return Err(de::Error::custom("Failed to parse bool")),
        }
        _ => return Err(de::Error::custom("Failed to parse bool")),
    })
}


fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: de::Deserializer<'de>,
{
    Ok(match de::Deserialize::deserialize(deserializer)? {
        Value::Number(n) => n.as_u64().unwrap(),
        Value::String(s) => s.parse::<u64>().unwrap(),
        _ => return Err(de::Error::custom("Failed to parse u64")),
    })
}
