// Copyright (c) 2025 Oleg Pavlenko and other contributors

const BASE_URL: &str = "https://api.yookassa.ru/v3";

mod error;
mod client;
mod auth;
mod api;
mod model;

pub mod models {
    pub use crate::model::*;
}

pub mod prelude {
    pub use super::error::*;
    pub use super::client::*;
    pub use super::auth::*;
}