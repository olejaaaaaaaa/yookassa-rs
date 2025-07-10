// Copyright (c) 2025 Oleg Pavlenko

use crate::{auth::Authentication, client::{YookassaClient, YookassaClientBuilder}, error::AnyError, BASE_URL};
use reqwest::{header::HeaderMap, Client, Method};
use erased_serde::Serialize;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use core::time::Duration;
use std::sync::Arc;

pub static DEFAULT_CLIENT: Lazy<Arc<Client>> = Lazy::new(|| {
    Arc::new(Client::builder()
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(1))
        .build()
        .unwrap())
});

pub struct YookassaRequestBuilder<'a> {
    client: Arc<Client>,
    auth: &'a dyn Authentication,
    path: String,
    method: Method,
    query: Option<&'a (dyn Serialize + Sync)>,
    json: Option<&'a (dyn Serialize + Sync)>,
    headers: Option<HeaderMap>,
}

impl<'a> YookassaRequestBuilder<'a> {

    pub fn new(client: Arc<Client>, path: String, method: Method, auth: &'a dyn Authentication) -> Self {
        Self {
            client,
            path,
            method,
            query: None,
            json: None,
            headers: None,
            auth: auth,
        }
    }

    pub fn query<T>(mut self, q: &'a T) -> Self
    where
        T: Serialize + Sync,
    {
        self.query = Some(q);
        self
    }

    pub fn json<T>(mut self, j: &'a T) -> Self
    where
        T: Serialize + Sync,
    {
        self.json = Some(j);
        self
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub async fn send<K: DeserializeOwned>(&self) -> Result<K, AnyError> {

        let client = self.client.request(self.method.clone(), format!("{}{}", BASE_URL, self.path));
        let mut request = self.auth.apply(client);


        if let Some(json) = self.json {
            request = request.json(json);
        }

        if let Some(query) = self.query {
            request = request.query(query)
        }

        if let Some(headers) = &self.headers {
            request = request.headers(headers.clone())
        }

        let resp = request.send().await?;
        let json = resp.json::<K>().await?;
        
        Ok(json)
    }
}

impl<S: Authentication> YookassaClient<S> {

    pub fn new(auth: S) -> Self {
        YookassaClientBuilder::default()
            .auth(auth)
            .build()
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self = Self {
            client: Arc::new(client),
            auth: self.auth
        };

        self
    }

    pub fn request<T: AsRef<str>>(&self, method: Method, path: T) -> YookassaRequestBuilder {
        YookassaRequestBuilder::new(self.client.clone(), path.as_ref().to_string(), method, &self.auth)
    }

}