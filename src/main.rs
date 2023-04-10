use dotenv::dotenv;
use serde_json::json;
use std::collections::HashMap;
use tokio;
mod yandex;
use crate::yandex::api::YandexApi;
use crate::yandex::station::Station;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let yandex_token = std::env::var("YANDEX_TOKEN").expect("Yandex Token not provided");

    let api = YandexApi::new(yandex_token);
    let devices = api.get_device_list().await;
    let device = devices.first().unwrap();
    let token = api.get_jwt_token(device).await;

    println!("Using device: {} ({})", device.name, device.id);
    let mut station = Station::new(device, token);

    station.connect().await;

    station.send(json!({
        "command": "sendText",
        "text": "Повтори за мной 'Локальный сервер подключен к станции'"
    }));

    station.handle_message().await;
}
