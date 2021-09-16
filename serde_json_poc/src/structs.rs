use std::time::{SystemTime, UNIX_EPOCH};

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize, Deserializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShadowRestoreResponse {
    date_time: u64,
    attributes: ShadowRestore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShadowRestore {
    #[serde(default)]
    code: String,
    #[serde(default)]
    firmware_version_code: u16,
    #[serde(default)]
    firmware_version_name: String,
    #[serde(default)]
    time_origin: u16,
    #[serde(default)]
    datetime: u128,
    #[serde(default)]
    firmware_update_status: u8,
    #[serde(default)]
    offline_setup_password: String,
    #[serde(default)]
    power_status: u8,
    #[serde(default)]
    power_voltage: f32,
    #[serde(default)]
    battery_voltage: f32,
    #[serde(default)]
    battery_level: f32,
    #[serde(default)]
    battery_low_level: f32,
    #[serde(default)]
    battery_low_level_reed_switch: f32,
    #[serde(default)]
    battery_low_level_pir_sensor: f32,
    #[serde(default)]
    battery_low_level_pir_photo_sensor: f32,
    #[serde(default)]
    battery_low_level_wireless_siren: f32,
    #[serde(default)]
    battery_low_level_remote_control: f32,
    #[serde(default)]
    connection_priority: u16,
    #[serde(default)]
    connection_mode: ConnectionMode,
    #[serde(default)]
    ethernet_connection_status: bool,
    #[serde(default)]
    ethernet_connection_speed: String,
    #[serde(default)]
    ethernet_ip_mode: NetworkMode,
    #[serde(default)]
    ethernet_ip: String,
    #[serde(default)]
    ethernet_gateway: String,
    #[serde(default)]
    ethernet_subnet_mask: String,
    #[serde(default)]
    ethernet_primary_dns: String,
    #[serde(default)]
    ethernet_secondary_dns: String,
    #[serde(default)]
    wifi_connection_status: bool,
    #[serde(default)]
    wifi_connection_speed: String,
    #[serde(default)]
    wifi_signal: u16,
    #[serde(default)]
    wifi_ip_mode: NetworkMode,
    #[serde(default)]
    wifi_ip: String,
    #[serde(default)]
    wifi_gateway: String,
    #[serde(default)]
    wifi_subnet_mask: String,
    #[serde(default)]
    wifi_primary_dns: String,
    #[serde(default)]
    wifi_secondary_dns: String,
    #[serde(default)]
    wifi_ssid: String,
    #[serde(default)]
    wifi_password: String,
    #[serde(default)]
    wifi_encryption: u8,
    #[serde(default)]
    gprs_connection_status: bool,
    #[serde(default)]
    gprs_connection_speed: String,
    #[serde(default)]
    gprs_apn1: String,
    #[serde(default)]
    gprs_apn_login1: String,
    #[serde(default)]
    gprs_apn_password1: String,
    #[serde(default)]
    gprs_apn_telecom1: String,
    #[serde(default)]
    gprs_apn2: String,
    #[serde(default)]
    gprs_apn_login2: String,
    #[serde(default)]
    gprs_apn_password2: String,
    #[serde(default)]
    gprs_apn_telecom2: String,
    #[serde(default)]
    gprs_apn3: String,
    #[serde(default)]
    gprs_apn_login3: String,
    #[serde(default)]
    gprs_apn_password3: String,
    #[serde(default)]
    gprs_apn_telecom3: String,
    #[serde(default)]
    gprs_apn4: String,
    #[serde(default)]
    gprs_apn_login4: String,
    #[serde(default)]
    gprs_apn_password4: String,
    #[serde(default)]
    gprs_apn_telecom4: String,
    #[serde(default)]
    gprs_apn5: String,
    #[serde(default)]
    gprs_apn_login5: String,
    #[serde(default)]
    gprs_apn_password5: String,
    #[serde(default)]
    gprs_apn_telecom5: String,
    #[serde(default)]
    gprs_telecom: String,
    #[serde(default)]
    gprs_iccid: String,
    #[serde(default)]
    gprs_signal: u8,
    #[serde(default)]
    config_phone_numbers: String,
    #[serde(default)]
    config_grouping_topic: String,
    #[serde(default)]
    log_level: u8,
    #[serde(default)]
    uptime_system: u128,
    #[serde(default)]
    uptime_connection: u128,
    #[serde(default = "true_default")]
    active_central_led: bool,
    #[serde(default)]
    default_partition: String,
    #[serde(default)]
    log_write: bool,
    #[serde(default)]
    central_mode: CentralMode,
    #[serde(default)]
    central_id: String,
    #[serde(deserialize_with = "tamper_status_from_bool", default)]
    tamper: TamperStatusShadow,
    #[serde(default = "volume_level_default")]
    volume_level: u64,
    #[serde(rename = "devices", default)]
    dev: Option<Vec<DeviceShadowRestore>>,
    #[serde(rename = "partitions", default)]
    par: Option<Vec<PartitionShadowRestore>>,
    #[serde(rename = "combinations", default)]
    com: Option<Vec<CombinationShadowRestore>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceShadowRestore {
    #[serde(rename="deviceCode", default)]
    code: String,
    #[serde(rename="deviceType", default)]
    device_type: DeviceType,
    #[serde(rename="deviceVersionCode", default)]
    version_code: u32,
    #[serde(rename="deviceBatteryLevel", default)]
    battery_level: f32,
    #[serde(rename="deviceCommunicationLevel", default)]
    communication_level: f32,
    #[serde(rename="deviceCommunicationStatus", default)]
    communication_status: bool,
    #[serde(rename="deviceHiddenZone", default)]
    hidden_zone: bool,
    #[serde(rename="deviceSensibilityLevel", default)]
    sensibility_level: u32,
    #[serde(rename="activeDeviceLed", default)]
    active_led: bool,
    #[serde(rename="deviceChimeSound", default)]
    chime_sound: u32,
    #[serde(rename="deviceStatus", default)]
    status: bool,
    #[serde(default)]
    device_id: String,
    #[serde(default)]
    accel_axis: u32,
    #[serde(default)]
    accel_duration: u32,
    #[serde(default)]
    accel_force: u32,
    #[serde(default)]
    accel_enable: bool,
    #[serde(default)]
    accel_debounce: u32,
    #[serde(default)]
    pir_sensibility: u32,
    #[serde(default)]
    pir_timeout: u32,
    #[serde(default)]
    pir_sample_rate: u32,
    #[serde(default)]
    gain: u32,
    #[serde(default)]
    tamper: bool,
    #[serde(default)]
    anti_sabotage: u8,
    #[serde(default)]
    battery_alarmed: bool,
    #[serde(default)]
    polling: u32,
    #[serde(default)]
    reporting: u32,
    #[serde(default)]
    p_cell_threshold: u32,
    #[serde(default)]
    compression: u8,
    #[serde(rename="flashmv", default)]
    flashmv: u32,
    #[serde(rename="pCellLowLightTH", default)]
    p_cell_lowlight_th: u32,
    #[serde(rename="pCellOutdoorTH", default)]
    p_cell_outdoor_th: u32,
    #[serde(default)]
    version: String,
    #[serde(default)]
    dry_contact: bool,
    #[serde(default)]
    silent: bool,
    #[serde(default)]
    dry_contact_opened: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PartitionShadowRestore {
    #[serde(rename="partitionCode", default)]
    code: String,
    #[serde(rename="partitionStatus", default)]
    status: PartitionStatus,
    #[serde(default)]
    disarmed_central_monitoring: bool,
    #[serde(default)]
    arm_wait_time: u32,
    #[serde(default)]
    automatic_rearm_time: u32,
    #[serde(default)]
    siren_time_on: u32,
    #[serde(default)]
    arm_with_opened_zone: bool,
    #[serde(default)]
    arm_fail_zone: bool,
    #[serde(default)]
    disarm_wait_time: u32,
    #[serde(default)]
    triggered: u8,
    #[serde(default)]
    forced_arm_expiration: u128,
    #[serde(default)]
    partition_id: String,
    #[serde(default)]
    number_triggers_disable_device: u16,
    #[serde(rename = "devices", default)]
    dev: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CombinationShadowRestore {
    #[serde(default)]
    code: String,
    #[serde(default)]
    time: u32,
    #[serde(default)]
    combination_id: String,
    #[serde(rename = "dev", default)]
    dev: Vec<String>,
}

#[derive(Clone, Hash, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
enum PartitionStatus {
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
enum CentralMode {
    DefaultMode = 1,
    MaintenanceMode = 2,
}

#[derive(Clone, Hash, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
enum TamperStatusShadow {
    Closed = 0,
    Opened = 1,
    Unknown = 2,
}

#[derive(Clone, Hash, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
enum DeviceType {
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
enum SensorType {
    Main = 1,
    TamperCase = 2,
    AntiSabotage = 3,
    Accelerometer = 4,
    DryContact = 5,
    Unknown = 0,
}

#[derive(Clone, Hash, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
enum NetworkMode {
    Unknown = 0,
    DHCP = 1,
    Static = 2,
}

#[derive(Clone, Hash, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
enum ConnectionMode {
    Unknown = 0,
    Ethernet = 1,
    Wifi = 2,
    GPRS = 3,
}

impl Default for ShadowRestore {
    fn default() -> ShadowRestore {
        ShadowRestore {
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
            battery_low_level: 10.0,
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
            central_id: String::default(),
            tamper: TamperStatusShadow::Closed,
            volume_level: 100,
            dev: Some(vec![]),
            par: Some(vec![PartitionShadowRestore::default()]),
            com: Some(vec![]),
        }
    }
}

impl Default for PartitionShadowRestore {
    fn default() -> Self {
        PartitionShadowRestore {
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
            dev: Some(Vec::new()),
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

fn tamper_status_from_bool <'de, D>(deserializer: D) -> Result<TamperStatusShadow, D::Error>
where
    D: Deserializer<'de>,
{
    match bool::deserialize(deserializer)? {
        true => Ok(TamperStatusShadow::Opened),
        _ => Ok(TamperStatusShadow::Closed),
    }
}