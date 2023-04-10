use crate::yandex::types::api::{Device, DeviceList, DeviceToken};
use reqwest;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

const GLAGOL_API_BASE_URL: &str = "https://quasar.yandex.net/glagol";

#[derive(Clone)]
pub struct YandexApi {
    oauth_token: String,
}

impl YandexApi {
    pub fn new(oauth_token: String) -> YandexApi {
        YandexApi { oauth_token }
    }

    pub async fn get_device_list(&self) -> Vec<Device> {
        let response = self.request::<DeviceList>("/device_list").await;
        response.devices
    }

    pub async fn get_jwt_token(&self, device: &Device) -> String {
        let url = format!(
            "/token?device_id={}&platform={}",
            device.id, device.platform
        );
        let response = self.request::<DeviceToken>(&url).await;

        response.token
    }

    pub async fn request<T>(&self, rest: &str) -> T
    where
        T: DeserializeOwned,
    {
        let mut headers = HeaderMap::new();
        let oauth = String::from("Oauth ") + &self.oauth_token;
        headers.insert("authorization", oauth.parse().unwrap());
        headers.insert("content-type", "application/json".parse().unwrap());

        let url = GLAGOL_API_BASE_URL.to_owned() + rest;

        let client = reqwest::Client::new();
        let response = client.get(url).headers(headers).send().await.unwrap();

        match response.status() {
            StatusCode::OK => match response.json::<T>().await {
                Ok(parsed) => return parsed,
                Err(e) => panic!(
                    "Hm, the response didn't match the shape we expected. {:?}",
                    e
                ),
            },

            other => {
                panic!("Uh oh! Something unexpected happened: {:?}", other);
            }
        }
    }
}
