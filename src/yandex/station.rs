use crate::yandex::types::api::Device;
use native_tls::{Certificate, TlsConnector};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{
    connect_async_tls_with_config, Connector, MaybeTlsStream, WebSocketStream,
};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestMessage {
    #[serde(rename = "conversationToken")]
    pub conversation_token: String,
    pub(crate) id: String,
    #[serde(rename = "sentTime")]
    pub sent_time: u64,
    pub payload: Value,
}

pub struct Station<'device> {
    device: &'device Device,
    token: String,
    ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl Station<'_> {
    pub fn new(device: &Device, token: String) -> Station {
        Station {
            device,
            token,
            ws: None,
        }
    }

    pub async fn connect(&mut self) {
        let wss_url = format!(
            "wss://{}:{}/",
            self.device.network_info.ip_addresses.first().unwrap(),
            self.device.network_info.external_port
        );
        println!("Connecting to: {}", wss_url);

        let cert = Certificate::from_pem((self.device.glagol.security.server_certificate).as_ref())
            .unwrap();
        // TODO find a solution to resolve the correct certificates
        let tls_connector = TlsConnector::builder()
            .add_root_certificate(cert)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let connector = Connector::NativeTls(tls_connector);

        let (ws_stream, _) = connect_async_tls_with_config(wss_url, None, Some(connector))
            .await
            .expect("Failed to connect");
        println!("Connected");
        self.ws = Option::from(ws_stream);
    }

    pub async fn handle_message(mut self) {
        let mut copy_ws = self.ws.take().unwrap();
        println!("Start listen input message...");
        let stream = copy_ws.get_mut();

        loop {
            let mut buf = vec![0; 1024];
            stream.read(&mut buf).await.expect("failed read");
            let json = String::from_utf8_lossy(&buf).to_string();
            println!("{}", json);

            let payload = serde_json::json!({
                "command": "pong"
            });

            self.send(payload).await;
        }
    }

    async fn send_raw(&mut self, buf: Vec<u8>) {
        let stream = self.ws.as_mut().unwrap().get_mut();
        stream
            .write(buf.as_slice())
            .await
            .expect("can't write message");
        stream
            .flush()
            .await
            .expect("something get wrong when send message");
        println!("stream flush");
    }

    pub async fn send(&mut self, payload: Value) {
        let message = RequestMessage {
            id: Uuid::new_v4().to_string(),
            payload,
            sent_time: 0,
            conversation_token: self.token.to_string(),
        };

        let json = serde_json::to_string(&message).unwrap();
        println!("{}", json);
        let buf = Message::text(json).into_data();
        self.send_raw(buf).await;
    }
}
