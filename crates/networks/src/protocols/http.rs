use crate::NetworkManager;
use anyhow::Result;
use reqwest::Client;

pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        HttpClient {
            client: Client::new(),
            base_url,
        }
    }
}

impl NetworkManager for HttpClient {
    type State = ();
    type RpcRequest = String;
    type RpcResponse = String;

    fn initialize(&mut self) -> Self::State {
        println!("Initializing HttpClient with base URL: {}", self.base_url);
    }

    async fn call_procedure(
        &mut self,
        procedure: &str,
        _request: &Self::RpcRequest,
    ) -> Result<Self::RpcResponse, String> {
        let url = format!("{}/{}", self.base_url, procedure);

        let res = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;

        Ok(res)
    }

    fn is_connection_oriented(&self) -> bool {
        false
    }

    fn close_connection(&mut self) {
        println!("HttpClient connection closed.");
    }
}
