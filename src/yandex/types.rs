pub mod api {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct DeviceToken {
        pub token: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct DeviceList {
        pub devices: Vec<Device>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Device {
        pub activation_code: i64,
        pub activation_region: String,
        pub config: Config,
        pub glagol: Glagol,
        pub id: String,
        pub name: String,
        #[serde(rename = "networkInfo")]
        pub network_info: NetworkInfo,
        pub platform: String,
        pub promocode_activated: bool,
        pub tags: Vec<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Config {
        pub equalizer: Equalizer,
        pub location: Location,
        pub name: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Equalizer {
        pub active_preset_id: String,
        pub bands: Vec<Band>,
        pub enabled: bool,
        #[serde(rename = "smartEnabled")]
        pub smart_enabled: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Band {
        pub freq: i64,
        pub gain: i64,
        pub width: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Location {
        pub latitude: f64,
        pub longitude: f64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Glagol {
        pub security: Security,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Security {
        pub server_certificate: String,
        pub server_private_key: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct NetworkInfo {
        pub external_port: i64,
        pub ip_addresses: Vec<String>,
        pub mac_addresses: Vec<String>,
        pub ts: i64,
        pub wifi_ssid: String,
    }
}
