use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize,Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::storage::Storage;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shadow {
    #[serde(skip)]
    pub storage: Option<Arc<Mutex<Storage>>>,
    #[serde(rename = "0", default)]
    pub code: String,
    #[serde(rename = "1", default)]
    pub firmware_version_code: u16,
    #[serde(rename = "2", default)]
    pub firmware_version_name: String,
    #[serde(rename = "3", default)]
    pub time_origin: u16,
    #[serde(rename = "5", default)]
    pub datetime: u128,
    #[serde(rename = "6", default)]
    pub firmware_update_status: u8,
    #[serde(rename = "8", default)]
    pub offline_setup_password: String,
    #[serde(rename = "9", default)]
    pub power_status: u8,
    #[serde(rename = "A", default, serialize_with = "round_serialize")]
    pub power_voltage: f32,
    #[serde(rename = "B", default, serialize_with = "round_serialize")]
    pub battery_voltage: f32,
    #[serde(rename = "C", default, serialize_with = "round_serialize")]
    pub battery_level: f32,
    #[serde(rename = "D", default, serialize_with = "round_serialize")]
    pub battery_low_level: f32,
    #[serde(rename = "E", default, serialize_with = "round_serialize")]
    pub battery_low_level_reed_switch: f32,
    #[serde(rename = "F", default, serialize_with = "round_serialize")]
    pub battery_low_level_pir_sensor: f32,
    #[serde(rename = "10", default, serialize_with = "round_serialize")]
    pub battery_low_level_pir_photo_sensor: f32,
    #[serde(rename = "11", default, serialize_with = "round_serialize")]
    pub battery_low_level_wireless_siren: f32,
    #[serde(rename = "12", default, serialize_with = "round_serialize")]
    pub battery_low_level_remote_control: f32,
    #[serde(rename = "13", default)]
    pub connection_priority: u16,
    #[serde(rename = "14", default)]
    pub connection_mode: ConnectionMode,
    #[serde(rename = "15", default)]
    pub ethernet_connection_status: bool,
    #[serde(rename = "16", default)]
    pub ethernet_connection_speed: String,
    #[serde(rename = "17", default)]
    pub ethernet_ip_mode: NetworkMode,
    #[serde(rename = "18", default)]
    pub ethernet_ip: String,
    #[serde(rename = "19", default)]
    pub ethernet_gateway: String,
    #[serde(rename = "1A", default)]
    pub ethernet_subnet_mask: String,
    #[serde(rename = "1B", default)]
    pub ethernet_primary_dns: String,
    #[serde(rename = "1C", default)]
    pub ethernet_secondary_dns: String,
    #[serde(rename = "1D", default)]
    pub wifi_connection_status: bool,
    #[serde(rename = "1E", default)]
    pub wifi_connection_speed: String,
    #[serde(rename = "1F", default)]
    pub wifi_signal: u16,
    #[serde(rename = "20", default)]
    pub wifi_ip_mode: NetworkMode,
    #[serde(rename = "21", default)]
    pub wifi_ip: String,
    #[serde(rename = "22", default)]
    pub wifi_gateway: String,
    #[serde(rename = "23", default)]
    pub wifi_subnet_mask: String,
    #[serde(rename = "24", default)]
    pub wifi_primary_dns: String,
    #[serde(rename = "25", default)]
    pub wifi_secondary_dns: String,
    #[serde(rename = "26", default)]
    pub wifi_ssid: String,
    #[serde(rename = "27", default)]
    pub wifi_password: String,
    #[serde(rename = "28", default)]
    pub wifi_encryption: u8,
    #[serde(rename = "29", default)]
    pub gprs_connection_status: bool,
    #[serde(rename = "2A", default)]
    pub gprs_connection_speed: String,
    #[serde(rename = "2C", default)]
    pub gprs_apn1: String,
    #[serde(rename = "2D", default)]
    pub gprs_apn_login1: String,
    #[serde(rename = "2E", default)]
    pub gprs_apn_password1: String,
    #[serde(rename = "2F", default)]
    pub gprs_apn_telecom1: String,
    #[serde(rename = "30", default)]
    pub gprs_apn2: String,
    #[serde(rename = "31", default)]
    pub gprs_apn_login2: String,
    #[serde(rename = "32", default)]
    pub gprs_apn_password2: String,
    #[serde(rename = "33", default)]
    pub gprs_apn_telecom2: String,
    #[serde(rename = "34", default)]
    pub gprs_apn3: String,
    #[serde(rename = "35", default)]
    pub gprs_apn_login3: String,
    #[serde(rename = "36", default)]
    pub gprs_apn_password3: String,
    #[serde(rename = "37", default)]
    pub gprs_apn_telecom3: String,
    #[serde(rename = "38", default)]
    pub gprs_apn4: String,
    #[serde(rename = "39", default)]
    pub gprs_apn_login4: String,
    #[serde(rename = "3A", default)]
    pub gprs_apn_password4: String,
    #[serde(rename = "3B", default)]
    pub gprs_apn_telecom4: String,
    #[serde(rename = "3C", default)]
    pub gprs_apn5: String,
    #[serde(rename = "3D", default)]
    pub gprs_apn_login5: String,
    #[serde(rename = "3E", default)]
    pub gprs_apn_password5: String,
    #[serde(rename = "3F", default)]
    pub gprs_apn_telecom5: String,
    #[serde(rename = "40", default)]
    pub gprs_telecom: String,
    #[serde(rename = "41", default)]
    pub gprs_iccid: String,
    #[serde(rename = "42", default)]
    pub gprs_signal: u8,
    #[serde(rename = "43", default)]
    pub config_phone_numbers: String,
    #[serde(rename = "45", default)]
    pub config_grouping_topic: String,
    #[serde(rename = "46", default)]
    pub log_level: u8,
    #[serde(rename = "47", default)]
    pub uptime_system: u128,
    #[serde(rename = "48", default)]
    pub uptime_connection: u128,
    #[serde(rename = "49", default = "true_default")]
    pub active_central_led: bool,
    #[serde(rename = "4A", default)]
    pub default_partition: String,
    #[serde(rename = "4B", default)]
    pub log_write: bool,
    #[serde(rename = "4C", default)]
    pub central_mode: CentralMode,
    #[serde(rename = "4D", default)]
    pub central_id: String,
    #[serde(rename = "4E", default)]
    pub tamper: TamperStatusShadow,
    #[serde(rename = "55", default = "volume_level_default")]
    pub volume_level: u64,
    #[serde(rename = "56", default = "true_default")]
    pub safe_call: bool,
    #[serde(rename = "57", default)]
    pub dial: bool,
    #[serde(rename = "58", default)]
    pub dial_number: u64,
    #[serde(rename = "59", default)]
    pub dial_delay: u64,
    #[serde(rename = "5A", default)]
    pub wwan_mtu: u16,
    #[serde(rename = "60", default)]
    pub demo: bool,
    #[serde(rename = "61", default = "true_default")]
    pub disarm_after_triggered: bool,
    #[serde(rename = "62", default)]
    pub wwan_ip: String,
    #[serde(rename = "dev", default)]
    pub dev: Vec<DeviceShadow>,
    #[serde(rename = "par", default)]
    pub par: Vec<PartitionShadow>,
    #[serde(rename = "com", default)]
    pub com: Vec<CombinationShadow>,
    #[serde(skip)]
    pub temp_devices: Vec<TempDevices>,
    #[serde(skip)]
    pub temp_remove_devices: Vec<(String, String)>,
    #[serde(skip)]
    pub opened_armed_devices: Vec<String>,
    #[serde(skip)]
    pub device_join_reporting: u32,
    #[serde(skip, default)]
    pub updates_disabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceShadow {
    #[serde(rename = "A0", default)]
    pub code: String,
    #[serde(rename = "A1", default)]
    pub device_type: DeviceType,
    #[serde(rename = "A2", default)]
    pub version_code: u32,
    #[serde(rename = "A3", default, serialize_with = "round_serialize")]
    pub battery_level: f32,
    #[serde(rename = "A4", default, serialize_with = "round_serialize")]
    pub communication_level: f32,
    #[serde(rename = "A5", default)]
    pub communication_status: bool,
    #[serde(rename = "A6", default)]
    pub hidden_zone: bool,
    #[serde(rename = "A7", default)]
    pub sensibility_level: u32,
    #[serde(rename = "A8", default)]
    pub active_led: bool,
    #[serde(rename = "A9", default)]
    pub chime_sound: u32,
    #[serde(rename = "AA", default)]
    pub status: bool,
    #[serde(rename = "AB", default)]
    pub device_id: String,
    #[serde(rename = "AC", default)]
    pub accel_axis: u32,
    #[serde(rename = "AD", default)]
    pub accel_duration: u32,
    #[serde(rename = "AE", default)]
    pub accel_force: u32,
    #[serde(rename = "AF", default)]
    pub accel_enable: bool,
    #[serde(rename = "B0", default)]
    pub accel_debounce: u32,
    #[serde(rename = "B1", default)]
    pub pir_sensibility: u32,
    #[serde(rename = "B2", default)]
    pub pir_timeout: u32,
    #[serde(rename = "B3", default)]
    pub pir_sample_rate: u32,
    #[serde(rename = "B4", default)]
    pub gain: u32,
    #[serde(rename = "B5", default)]
    pub tamper: bool,
    #[serde(rename = "B6", default)]
    pub anti_sabotage: u8,
    #[serde(rename = "B7", default)]
    pub battery_alarmed: bool,
    #[serde(rename = "B8", default)]
    pub polling: u32,
    #[serde(rename = "B9", default)]
    pub reporting: u32,
    #[serde(rename = "BA", default)]
    pub p_cell_threshold: u32,
    #[serde(rename = "BB", default)]
    pub compression: u8,
    #[serde(rename = "BC", default)]
    pub flash_mv: u32,
    #[serde(rename = "BD", default)]
    pub p_cell_lowlight_th: u32,
    #[serde(rename = "BE", default)]
    pub p_cell_outdoor_th: u32,
    #[serde(rename = "BF", default)]
    pub version: String,
    #[serde(rename = "F0", default)]
    pub dry_contact: bool,
    #[serde(rename = "F1", default)]
    pub silent: bool,
    #[serde(rename = "F2", default)]
    pub dry_contact_opened: bool,
    #[serde(rename = "F3", default)]
    pub fw_version: String,
    #[serde(rename = "F4", default)]
    pub hw_version: String,
    #[serde(rename = "F5", default)]
    pub rssi: i32,
    #[serde(rename = "F6", default)]
    pub last_communication: u128,
    #[serde(rename = "F7", default, serialize_with = "round_serialize")]
    pub device_battery_voltage: f32,
    #[serde(rename = "F8", default, serialize_with = "round_serialize")]
    pub device_temperature: f32,
    #[serde(rename = "F9", default)]
    pub hidden_zone_from: u64,
    #[serde(rename = "FA", default)]
    pub hidden_zone_to: u64,
    #[serde(rename = "FB", default)]
    pub crystal_capacitance: u8,
    #[serde(skip, default)]
    pub trigger_count: u16,
    #[serde(skip, default)]
    pub should_configure: bool,
    #[serde(skip, default)]
    pub send_photo_enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartitionShadow {
    #[serde(rename = "C0", default)]
    pub code: String,
    #[serde(rename = "C1", default)]
    pub status: PartitionStatus,
    #[serde(rename = "C2", default)]
    pub disarmed_central_monitoring: bool,
    #[serde(rename = "C3", default)]
    pub arm_wait_time: u32,
    #[serde(rename = "C4", default)]
    pub automatic_rearm_time: u32,
    #[serde(rename = "C5", default)]
    pub siren_time_on: u32,
    #[serde(rename = "C6", default)]
    pub arm_with_opened_zone: bool,
    #[serde(rename = "C7", default)]
    pub arm_fail_zone: bool,
    #[serde(rename = "C8", default)]
    pub disarm_wait_time: u32,
    #[serde(rename = "C9", default)]
    pub triggered: u8,
    #[serde(rename = "CA", default)]
    pub forced_arm_expiration: u128,
    #[serde(rename = "CB", default)]
    pub partition_id: String,
    #[serde(rename = "CC", default)]
    pub number_triggers_disable_device: u16,
    #[serde(rename = "CD", default)]
    pub partial: bool,
    #[serde(rename = "CE", default)]
    pub last_arm_datetime: Option<u128>,
    #[serde(default)]
    pub partial_arm_devices: Vec<String>,
    #[serde(rename = "dev", default)]
    pub dev: Vec<String>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CombinationShadow {
    #[serde(rename = "E0", default)]
    pub code: String,
    #[serde(rename = "E1", default)]
    pub time: u32,
    #[serde(rename = "E2", default)]
    pub combination_id: Uuid,
    #[serde(rename = "dev", default)]
    pub dev: Vec<String>,
    #[serde(skip, default)]
    pub timestamp: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TempDevices {
    pub code: String,
    pub id: String,
}

#[derive(Clone, Hash, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PartitionStatus {
    #[serde(rename = "A")]
    Armed,
    #[serde(rename = "D")]
    Disarmed,
    #[serde(rename = "S")]
    Stayed,
    #[serde(rename = "T")]
    Triggered,
}

#[derive(Clone, Hash, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum CentralMode {
    DefaultMode = 1,
    MaintenanceMode = 2,
    Standby = 3,
    Demo = 4,
}

#[derive(Clone, Hash, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum TamperStatusShadow {
    Closed = 0,
    Opened = 1,
    Unknown = 2,
}

#[derive(Clone, Hash, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum DeviceType {
    None = 255,
    Control = 0,
    Pir = 1,
    Camera = 2,
    Switch = 3,
    Door = 4,
    Siren = 5,
}

#[derive(Clone, Hash, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum SensorType {
    Main = 1,
    TamperCase = 2,
    AntiSabotage = 3,
    Accelerometer = 4,
    DryContact = 5,
    Unknown = 0,
}

#[derive(Clone, Hash, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum NetworkMode {
    Unknown = 0,
    DHCP = 1,
    Static = 2,
}

#[derive(Clone, Hash, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum ConnectionMode {
    Unknown = 0,
    Ethernet = 1,
    Wifi = 2,
    GPRS = 3,
}

impl Default for Shadow {
    fn default() -> Shadow {
        Shadow {
            storage: None,
            code: String::default(),
            firmware_version_code: 0,
            firmware_version_name: String::default(),
            time_origin: 1,
            datetime: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            firmware_update_status: 1,
            offline_setup_password: String::from("rdcpass"),
            power_status: 100,
            power_voltage: 100.0,
            battery_voltage: 100.0,
            battery_level: 100.0,
            battery_low_level: 25.0,
            battery_low_level_reed_switch: 10.0,
            battery_low_level_pir_sensor: 10.0,
            battery_low_level_pir_photo_sensor: 10.0,
            battery_low_level_wireless_siren: 10.0,
            battery_low_level_remote_control: 10.0,
            connection_priority: 1,
            connection_mode: ConnectionMode::Ethernet,
            ethernet_connection_status: false,
            ethernet_connection_speed: String::from("100Mbps"),
            ethernet_ip_mode: NetworkMode::DHCP,
            ethernet_ip: String::default(),
            ethernet_gateway: String::default(),
            ethernet_subnet_mask: String::default(),
            ethernet_primary_dns: String::default(),
            ethernet_secondary_dns: String::default(),
            wifi_connection_status: false,
            wifi_connection_speed: String::from("100Mbps"),
            wifi_signal: 1,
            wifi_ip_mode: NetworkMode::DHCP,
            wifi_ip: String::default(),
            wifi_gateway: String::default(),
            wifi_subnet_mask: String::default(),
            wifi_primary_dns: String::default(),
            wifi_secondary_dns: String::default(),
            wifi_ssid: String::default(),
            wifi_password: String::default(),
            wifi_encryption: 1,
            gprs_connection_status: false,
            gprs_connection_speed: String::default(),
            gprs_apn1: String::default(),
            gprs_apn_login1: String::default(),
            gprs_apn_password1: String::default(),
            gprs_apn_telecom1: String::default(),
            gprs_apn2: String::default(),
            gprs_apn_login2: String::default(),
            gprs_apn_password2: String::default(),
            gprs_apn_telecom2: String::default(),
            gprs_apn3: String::default(),
            gprs_apn_login3: String::default(),
            gprs_apn_password3: String::default(),
            gprs_apn_telecom3: String::default(),
            gprs_apn4: String::default(),
            gprs_apn_login4: String::default(),
            gprs_apn_password4: String::default(),
            gprs_apn_telecom4: String::default(),
            gprs_apn5: String::default(),
            gprs_apn_login5: String::default(),
            gprs_apn_password5: String::default(),
            gprs_apn_telecom5: String::default(),
            gprs_telecom: String::default(),
            gprs_iccid: String::default(),
            gprs_signal: 1,
            config_phone_numbers: String::default(),
            config_grouping_topic: String::default(),
            log_level: 6,
            uptime_system: 1,
            uptime_connection: 1,
            active_central_led: true,
            default_partition: String::from("1"),
            log_write: true,
            central_mode: CentralMode::DefaultMode,
            central_id: Uuid::default().to_string(),
            tamper: TamperStatusShadow::Closed,
            volume_level: 100,
            safe_call: true,
            dial: true,
            dial_number: 0,
            dial_delay: 0,
            wwan_mtu: 1342,
            demo: false,
            dev: vec![],
            par: vec![PartitionShadow::default()],
            com: vec![],
            temp_devices: vec![],
            temp_remove_devices: vec![],
            opened_armed_devices: vec![],
            device_join_reporting: 0,
            updates_disabled: false,
            disarm_after_triggered: true,
            wwan_ip: String::default(),
        }
    }
}

impl Default for PartitionShadow {
    fn default() -> Self {
        PartitionShadow {
            code: String::from("1"),
            status: PartitionStatus::Disarmed,
            disarmed_central_monitoring: false,
            arm_wait_time: 0,
            automatic_rearm_time: 0,
            siren_time_on: 60,
            arm_with_opened_zone: false,
            arm_fail_zone: false,
            disarm_wait_time: 60,
            triggered: 1,
            forced_arm_expiration: 0,
            partition_id: Uuid::new_v4().to_string(),
            number_triggers_disable_device: 10,
            partial: false,
            last_arm_datetime: Some(0),
            partial_arm_devices: Vec::new(),
            dev: Vec::new(),
        }
    }
}

impl Default for TamperStatusShadow {
    fn default() -> TamperStatusShadow {
        TamperStatusShadow::Closed
    }
}

impl Default for CentralMode {
    fn default() -> Self {
        CentralMode::DefaultMode
    }
}

impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::None
    }
}

impl Default for PartitionStatus {
    fn default() -> Self {
        PartitionStatus::Disarmed
    }
}

impl Default for NetworkMode {
    fn default() -> Self {
        NetworkMode::DHCP
    }
}

impl Default for ConnectionMode {
    fn default() -> Self {
        ConnectionMode::Ethernet
    }
}

impl From<String> for NetworkMode {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "dhcp" => NetworkMode::DHCP,
            "static" => NetworkMode::Static,
            _ => NetworkMode::Unknown,
        }
    }
}

impl From<String> for ConnectionMode {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "lan" | "lan0" => ConnectionMode::Ethernet,
            "wlan" | "wlan0" => ConnectionMode::Wifi,
            "wwan" | "wwan0" => ConnectionMode::GPRS,
            _ => ConnectionMode::Unknown,
        }
    }
}

fn volume_level_default() -> u64 {
    100
}

fn true_default() -> bool {
    true
}

fn round_serialize<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_f32((x * 1000.).round() / 1000.)
}
