use serde::Deserialize;
use serde::de;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Robot {
    pub litter_robot_id: String,
    pub litter_robot_serial: String,
    pub litter_robot_nickname: String,
    pub device_type: String,            // "iot"
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
    pub is_manual_reset: bool,
    pub saved_is_manual_reset: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(rename = "previousDFITriggered")]
    pub previous_dfi_triggered: bool,
    #[serde(deserialize_with = "deserialize_u64")]
    #[serde(rename = "DFICycleCount")]
    pub dfi_cycle_count: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub saved_cycle_count: u64,
    pub clean_cycle_wait_time_minutes: String, // "F" -
    pub cycles_after_drawer_full: u64,
    #[serde(deserialize_with = "deserialize_bool")]
    pub night_light_active: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub panel_lock_active: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub sleep_mode_active: bool,
    pub sleep_mode_time: u64,
    pub power_status: String,               // battery vs ac?
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
    #[serde(deserialize_with = "deserialize_u64")]
    pub baseline_cycle_count: u64,
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





fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => Err(de::Error::unknown_variant(s, &["0", "1"]))
    }
}

fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    s.parse().map_err(|_| de::Error::custom(format!("Expected a u64, found {}", s)))
}
