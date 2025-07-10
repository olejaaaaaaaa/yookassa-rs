// Copyright (c) 2025 Oleg Pavlenko and other contributors

use crate::{
    auth::Authentication,
    client::{
        YookassaClient,
        YookassaClientBuilder
    },
    error::YookassaError,
    BASE_URL
};

use reqwest::{
    header::HeaderMap,
    Client,
    Method, StatusCode
};

use erased_serde::Serialize;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use core::time::Duration;

pub static DEFAULT_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(1))
        .build()
        .unwrap()
});

pub struct YookassaRequestBuilder<'a> {
    client: Client,
    auth: &'a dyn Authentication,
    path: String,
    method: Method,
    query: Option<&'a (dyn Serialize)>,
    json: Option<&'a (dyn Serialize)>,
    headers: Option<HeaderMap>,
}

impl<'a> YookassaRequestBuilder<'a> {

    pub fn new(client: Client, path: String, method: Method, auth: &'a dyn Authentication) -> Self {
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
        T: Serialize,
    {
        self.query = Some(q);
        self
    }

    pub fn json<T>(mut self, j: &'a T) -> Self
    where
        T: Serialize,
    {
        self.json = Some(j);
        self
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub async fn send<Deserialize: DeserializeOwned>(&self) -> Result<Deserialize, YookassaError> {

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
                    let json = resp.json::<Deserialize>().await;
                    match json {
                        Ok(json) => Ok(json),
                        Err(err) => Err(YookassaError::Json(err))
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
            client: client,
            auth: self.auth
        };

        self
    }

    pub fn request<T: AsRef<str>>(&self, method: Method, path: T) -> YookassaRequestBuilder {
        YookassaRequestBuilder::new(self.client.clone(), path.as_ref().to_string(), method, &self.auth)
    }

}