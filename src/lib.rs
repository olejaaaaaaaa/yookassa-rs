// Copyright (c) 2025 Oleg Pavlenko

pub const BASE_URL: &str = "https://api.yookassa.ru/v3";

mod error;
mod client;
mod auth;
mod api;
mod data;

pub use data::*;

pub mod prelude {
    pub use super::error::*;
    pub use super::client::*;
    pub use super::auth::*;
}