// Copyright (c) 2025 Oleg Pavlenko and other contributors

use reqwest::Client;
use crate::auth::Authentication;
use crate::api::client::DEFAULT_CLIENT;

#[derive(Clone)]
pub struct YookassaClient<S: Authentication> {
    pub client: Client,
    pub auth: S
}

///
/// Builder for custom 
/// 
/// Example:
/// ```
/// ```
/// 
/// 
#[derive(Default)]
pub struct YookassaClientBuilder<S: Authentication> {
    pub client: Option<Client>,
    pub auth: Option<S>
}

impl<S: Authentication> YookassaClientBuilder<S> {
    pub fn auth(mut self, auth: S) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn default() -> Self {
        Self { client: None, auth: None }
    }

    pub fn build(self) -> YookassaClient<S> {
        YookassaClient { 
            client: self.client.unwrap_or(DEFAULT_CLIENT.clone()),
            auth: self.auth.expect("Auth is not set") 
        }
    }
}







