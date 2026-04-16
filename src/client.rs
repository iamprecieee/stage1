use crate::errors::Result;

#[derive(Clone, Debug)]
pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn init() -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
        })
    }

    pub fn get(&self) -> reqwest::Client {
        self.client.clone()
    }
}
