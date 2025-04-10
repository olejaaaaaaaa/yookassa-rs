// Copyright (c) 2025 Oleg Pavlenko

use reqwest::Client;
use crate::auth::Authentication;
use std::sync::Arc;
use crate::api::client::DEFAULT_CLIENT;

pub struct YookassaClient<S: Authentication> {
    pub client: Arc<Client>,
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
    pub client: Option<Arc<Client>>,
    pub auth: Option<S>
}

impl<S: Authentication> YookassaClientBuilder<S> {
    pub fn auth(mut self, auth: S) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(Arc::new(client));
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







