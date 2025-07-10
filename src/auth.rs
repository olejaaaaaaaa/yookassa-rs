// Copyright (c) 2025 Oleg Pavlenko and other contributors

use reqwest::RequestBuilder;

/// Implement this trait to add a new authentication method.
///
/// # Examples
///
/// ```
/// use reqwest::RequestBuilder;
/// use uuid::Uuid;
///
/// use yookassa_rs::prelude::Authentication;
///
/// struct MyAuth {
///     id: Uuid,
///     key: Uuid
/// }
///
/// impl Authentication for MyAuth {
///     fn apply(&self, request: RequestBuilder) -> RequestBuilder {
///         request.basic_auth(self.id.clone().to_string(), Some(self.key.clone().to_string()))
///     }
/// }
///
/// ```
pub trait Authentication {
    fn apply(&self, request: RequestBuilder) -> RequestBuilder;
}

/// Standart basic auth
///
/// # Examples
/// ```
/// fn main() {
///     use yookassa_rs::prelude::{BasicAuth, YookassaClient, YookassaClientBuilder};
///     let shop_id = "12456";
///     let secret_key = "test_1HtqeKIGTQan9ODGjHd3IXTa5v1U34TU0JDiqUbsGj4";
///     let client: YookassaClient<BasicAuth> = YookassaClientBuilder::new(BasicAuth::new(secret_key, shop_id))
///         .build();
/// }
/// ```
#[derive(Clone)]
pub struct BasicAuth {
    pub secret_key: String,
    pub shop_id: String
}

impl BasicAuth {
    pub fn new<S: Into<String>>(secret_key: S, shop_id: S) -> Self {
        Self {
            secret_key: secret_key.into(),
            shop_id: shop_id.into()
        }
    }
}

impl Authentication for BasicAuth {
    fn apply(&self, request: RequestBuilder) -> RequestBuilder {
        request.basic_auth(self.shop_id.clone(), Some(self.secret_key.clone()))
    }
}

/// Struct contain a OAuth 
///
/// # Examples
/// ```
/// fn main() {
///     use yookassa_rs::prelude::{OAuth, YookassaClient, YookassaClientBuilder};
///     let token = "679hfddjk-238238dsg-123fdfhr";
///     let client: YookassaClient<OAuth> = YookassaClientBuilder::default()
///         .auth(OAuth::new(token))
///         .build();
/// }
/// ```
pub struct OAuth {
    pub token: String
}

impl OAuth {
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            token: token.into()
        }
    }
}

impl Authentication for OAuth {
    fn apply(&self, request: RequestBuilder) -> RequestBuilder {
        request.bearer_auth(self.token.clone())
    }
}



