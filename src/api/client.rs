// Copyright (c) 2025 Oleg Pavlenko

use crate::{auth::Authentication, client::{YookassaClient, YookassaClientBuilder}, error::YookassaError, BASE_URL};
use reqwest::{header::HeaderMap, Client, Method, StatusCode};
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
    path: &'a str,
    method: Method,
    query: Option<&'a dyn Serialize>,
    json: Option<&'a dyn Serialize>,
    headers: Option<HeaderMap>,
}

impl<'a> YookassaRequestBuilder<'a> {

    pub fn new(client: Arc<Client>, path: &'a str, method: Method, auth: &'a dyn Authentication) -> Self {
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

    pub fn query<T: Serialize>(mut self, query: &'a T) -> Self {
        self.query = Some(query);
        self
    }

    pub fn json<T: Serialize>(mut self, json: &'a T) -> Self {
        self.json = Some(json);
        self
    }

    pub fn headers<K, V>(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub async fn send<K: DeserializeOwned>(&self) -> Result<K, YookassaError> {

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

        let resp = request.send().await;
        match resp {
            Ok(resp) => { 
                if resp.status() == StatusCode::OK {
                    let json = resp.json::<K>().await;
                    if let Ok(json) = json {
                        return Ok(json);
                    } else {
                        return Err(YookassaError::Reqwest(json.err().unwrap()));
                    }
                } else {
                    return Err(YookassaError::Code(resp.status()));
                }
            },
            Err(err) => { Err(YookassaError::Reqwest(err)) }
        }

        
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

    pub fn request(&self, method: Method, path: &'static str) -> YookassaRequestBuilder {
        YookassaRequestBuilder::new(self.client.clone(), path, method, &self.auth)
    }

}